use crate::components::MatchEvent;
use crate::homography::{find_homography, HomographyMatrix};
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




pub struct EstimationLabel(pub String);
impl std::fmt::Display for EstimationLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ArrsacEstimation;
pub fn estimate_homography_with_arrsac(
    mut commands: Commands,
    mut ev_matches: EventReader<MatchEvent>,
    mut task: Local<Option<Task<Option<HomographyMatrix>>>>,
    mut estimation: Query<(&mut Option<HomographyMatrix>, &ArrsacEstimation)>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    if task.is_some() {
        if let Some(hm) = future::block_on(future::poll_once(task.as_mut().unwrap())) {
            if let Ok((mut _hm, _)) = estimation.single_mut() {
                *_hm = hm;
            } else {
                commands.spawn().insert_bundle((hm, ArrsacEstimation, EstimationLabel("With ARRSAC".into())));
            }
            *task = None;
        }
    }

    if task.is_none() {
        for MatchEvent(matches) in ev_matches.iter() {
            let matches = matches.clone();
            *task = Some(thread_pool.spawn(async move { find_homography(&matches) }));

            break;
        }
    }
}

pub struct OpenCVEstimation;
pub fn estimate_homography_with_opencv(
    mut commands: Commands,
    mut ev_matches: EventReader<MatchEvent>,
    mut task: Local<Option<Task<Option<HomographyMatrix>>>>,
    mut estimation: Query<(&mut Option<HomographyMatrix>, &OpenCVEstimation)>,
    thread_pool: Res<AsyncComputeTaskPool>,
) {
    if task.is_some() {
        if let Some(hm) = future::block_on(future::poll_once(task.as_mut().unwrap())) {
            if let Ok((mut _hm, _)) = estimation.single_mut() {
                *_hm = hm;
            } else {
                commands.spawn().insert_bundle((hm, OpenCVEstimation, EstimationLabel("With OpenCV".into())));
            }
            *task = None;
        }
    }

    if task.is_none() {
        for MatchEvent(matches) in ev_matches.iter() {
            let matches = matches.clone();
            *task = Some(thread_pool.spawn(async move {
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
                    0,
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
            }));

            break;
        }
    }
}
