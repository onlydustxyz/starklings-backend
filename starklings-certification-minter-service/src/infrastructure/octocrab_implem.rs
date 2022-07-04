use crate::domain::{
	errors::Error,
	traits::{GithubUserGetter, GithubUserId},
};
use async_trait::async_trait;
use octocrab::Octocrab;

pub(crate) struct OctocrabInstance {
	inner: Octocrab,
}

impl OctocrabInstance {
	pub fn new(token: String) -> Result<Self, octocrab::Error> {
		Ok(Self {
			inner: Octocrab::builder().personal_token(token).build()?,
		})
	}
}

#[async_trait]
impl GithubUserGetter for OctocrabInstance {
	async fn get_user_id(&self, _token: &str) -> Result<GithubUserId, Error> {
		Ok(*self.inner.current().user().await.map_err(|e| Error::Github(e.to_string()))?.id)
	}
}
