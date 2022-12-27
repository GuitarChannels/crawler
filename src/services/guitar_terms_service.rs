use crate::repos::non_guitar_channel_repo::NonGuitarChannelRepository;

pub struct GuitarTermResult {
    pub has_guitar_term: bool,
    pub is_blacklisted: bool,
}

pub struct GuitarTermsService {
    guitar_terms: Vec<String>,
    blacklisted_channel_ids: Vec<String>,
    non_guitar_channel_repo: NonGuitarChannelRepository,
}

impl GuitarTermsService {
    pub fn new(
        guitar_terms: Vec<String>,
        blacklisted_channel_ids: Vec<String>,
        non_guitar_channel_repo: NonGuitarChannelRepository,
    ) -> GuitarTermsService {
        GuitarTermsService {
            guitar_terms,
            blacklisted_channel_ids,
            non_guitar_channel_repo,
        }
    }

    pub async fn is_not_listed_as_non_guitar_channel(&self, channel_id: &str) -> bool {
        let non_guitar_channel_exists = self
            .non_guitar_channel_repo
            .exists(channel_id)
            .await
            .unwrap_or(false);

        !non_guitar_channel_exists
    }

    pub async fn has_guitar_term(
        &self,
        channel_id: &str,
        channel_title: &str,
        channel_description: &str,
        ignore_guitar_terms: bool,
    ) -> GuitarTermResult {
        let mut has_guitar_term = false;
        let mut is_blacklisted = false;

        for term in &self.guitar_terms {
            if channel_title.to_lowercase().contains(term)
                || channel_description.to_lowercase().contains(term)
            {
                has_guitar_term = true;
                break;
            }
        }

        if has_guitar_term == false && ignore_guitar_terms == false {
            self.non_guitar_channel_repo.upsert(&channel_id).await;
        }

        if ignore_guitar_terms == true {
            has_guitar_term = true;
        }

        if self
            .blacklisted_channel_ids
            .contains(&channel_id.to_string())
        {
            has_guitar_term = false;
            is_blacklisted = true;
            //self.delete_channel(channel_id).await.unwrap();
        }

        GuitarTermResult {
            has_guitar_term,
            is_blacklisted,
        }
    }
}
