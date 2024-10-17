use std::{env, sync::Arc};

use candid::Principal;
use dotenv_codegen::dotenv;
use ic_agent::{agent::AgentBuilder, Agent, Identity};


#[derive(Clone)]
pub struct AgentWrapper(Agent);

impl AgentWrapper {
    pub fn build(builder_func: impl FnOnce(AgentBuilder) -> AgentBuilder) -> Self {
        // dotenv::dotenv().ok();


        let is_dev = dotenv!("BACKEND") == "LOCAL";

        
        let mut builder =  Agent::builder().with_url(if is_dev {crate::consts::local::AGENT_URL } else {crate::consts::remote::AGENT_URL});
        builder = builder_func(builder);
        Self(builder.build().unwrap())
    }

    pub async fn get_agent(&self) -> &Agent {
        let agent = &self.0;
        #[cfg(any(feature = "local-bin", feature = "local-lib"))]
        {
            agent
                .fetch_root_key()
                .await
                .expect("AGENT: fetch_root_key failed");
        }
        agent
    }

    pub fn set_arc_id(&mut self, id: Arc<impl Identity + 'static>) {
        self.0.set_arc_identity(id);
    }

    pub fn principal(&self) -> Result<Principal, String> {
        self.0.get_principal()
    }
}
