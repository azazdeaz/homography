use crate::components::MatchEvent;
use crate::homography::{find_homography, HomographyMatrix};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;

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
                commands.spawn().insert_bundle((hm, ArrsacEstimation));
            }
            *task = None;
        }
    }
    
    if task.is_none() {
        for MatchEvent(matches) in ev_matches.iter() {
            let matches = matches.clone();
            *task = Some(thread_pool.spawn(async move { println!("task done");return find_homography(&matches) }));
            
            break;
        }
    }
}
