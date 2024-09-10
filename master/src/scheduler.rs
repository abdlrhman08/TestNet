use std::sync::Arc;

use net_interface::{JobQueue, NodeMap};
use tokio::sync::Mutex;

pub struct Scheduler {
    pub job_queue: Arc<Mutex<JobQueue>>,
    node_data: NodeMap,
}

impl Scheduler {
    pub fn new(job_queue: &Arc<Mutex<JobQueue>>, node_data: &NodeMap) -> Self {
        let job_queue = Arc::clone(job_queue);
        let node_data = Arc::clone(node_data);

        Self {
            job_queue,
            node_data,
        }
    }

    pub async fn schedule_jobs(&self) {
        let node_data = &mut *self.node_data.lock().await;
        //TODO!: make this circular loop and set the state of the node as busy
        for (_, node) in node_data.iter_mut() {
            if !node.busy_state {
                // is it healthier to take the lock once even if there is the
                // possibility that we won't use it, or take it everytime we are
                // sure we will use it
                let job_queue = &mut *self.job_queue.lock().await;
                let upcoming_job = job_queue.get_upcoming();
                if let None = upcoming_job {
                    break;
                }
                node.get_sender()
                    .send(Ok(upcoming_job.unwrap()))
                    .await
                    .unwrap();

                //TODO!: check if sent
                node.busy_state = true
            }
        }
    }
}
