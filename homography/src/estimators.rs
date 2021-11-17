use crate::components::MatchEvent;
use crate::homography::{find_homography, run_homography_kernel, HomographyMatrix};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use itertools::Itertools;
use opencv::{
    self,
    core::{Mat_, Point2f, ToInputArray},
    prelude::{Mat, MatTrait, MatTraitConst},
    types::VectorOfPoint2f,
};
use std::time::{Duration, Instant};

pub struct EstimationLabel(pub String);
impl std::fmt::Display for EstimationLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn estimate_homography_with_arrsac(
    use_sc: Local<bool>,
    mut commands: Commands,
    mut ev_matches: EventReader<MatchEvent>,
    mut task: Local<Option<(Task<Option<HomographyMatrix>>, Instant)>>,
    mut estimation_entity: Local<Option<Entity>>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    if task.is_some() {
        let (t, started_at) = task.as_mut().unwrap();
        if let Some(hm) = future::block_on(future::poll_once(t)) {
            let time = started_at.elapsed();
            if let Some(estimation_entity) = *estimation_entity {
                commands.entity(estimation_entity).insert_bundle((hm, time));
            } else {
                let label = if *use_sc { "With ARRSAC" } else { "Simple" };
                let label = EstimationLabel(label.into());
                *estimation_entity = Some(commands.spawn().insert_bundle((hm, label, time)).id());
            }
            *task = None;
        }
    }

    if task.is_none() {
        for MatchEvent(matches) in ev_matches.iter() {
            let matches = matches.clone();
            let use_sc = *use_sc;
            *task = Some((
                thread_pool.spawn(async move {
                    if use_sc {
                        find_homography(&matches)
                    } else {
                        if let Ok(h) = run_homography_kernel(matches) {
                            Some(HomographyMatrix(h))
                        }
                        else {
                            None
                        }
                    }
                }),
                Instant::now(),
            ));

            break;
        }
    }
}

pub struct OpenCVEstimation;
pub fn estimate_homography_with_opencv(
    mut commands: Commands,
    mut ev_matches: EventReader<MatchEvent>,
    mut task: Local<Option<(Task<Option<HomographyMatrix>>, Instant)>>,
    mut estimation: Query<(
        &mut Option<HomographyMatrix>,
        &mut Duration,
        &OpenCVEstimation,
    )>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    if task.is_some() {
        let (t, started_at) = task.as_mut().unwrap();
        if let Some(hm) = future::block_on(future::poll_once(t)) {
            if let Ok((mut _hm, mut time, _)) = estimation.single_mut() {
                *_hm = hm;
                *time = started_at.elapsed();
            } else {
                commands.spawn().insert_bundle((
                    hm,
                    OpenCVEstimation,
                    EstimationLabel("With OpenCV".into()),
                    started_at.elapsed(),
                ));
            }
            *task = None;
        }
    }

    if task.is_none() {
        for MatchEvent(matches) in ev_matches.iter() {
            let matches = matches.clone();
            *task = Some((
                thread_pool.spawn(async move {
                    let (src, dst): (VectorOfPoint2f, VectorOfPoint2f) = matches
                        .iter()
                        .map(|m| {
                            (
                                Point2f::new(m.0.coords.x as f32, m.0.coords.y as f32),
                                Point2f::new(m.1.coords.x as f32, m.1.coords.y as f32),
                            )
                        })
                        .unzip();

                    let opencv_res = opencv::calib3d::find_homography(
                        &src.input_array().unwrap(),
                        &dst.input_array().unwrap(),
                        &mut Mat::default(),
                        opencv::calib3d::RANSAC,
                        3.,
                    );

                    if let Ok(mut res) = opencv_res {
                        let values = (0..res.total().unwrap() as i32)
                            .map(|i| res.at_mut::<f64>(i).unwrap().clone())
                            .collect_vec();
                        let mat = nalgebra::Matrix3::from_row_slice(values.as_slice());
                        Some(HomographyMatrix(mat))
                    } else {
                        None
                    }
                }),
                Instant::now(),
            ));

            break;
        }
    }
}
