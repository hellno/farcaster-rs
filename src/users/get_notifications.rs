use crate::{types::user::notifications::NotifRoot, Farcaster};

impl Farcaster {
    #[allow(unused_assignments)]
    /// # Get all notifications of a user
    ///
    /// ## Arguments
    ///
    /// * `self: &Farcaster`
    ///     - Takes in a type of Farcaster which is created at the start with ``Farcaster::new("client");``
    ///
    /// * `username: Option<String>`
    ///     - The username you want to fetch casts for
    ///     - Optional
    ///
    /// * `address: Option<String>`
    ///     - The address for the user you want to get
    ///     - Optional
    ///
    /// ## Usage
    /// ```
    /// let farcaster = Farcaster::new("");
    ///
    /// let notifications = farcaster.get_notifications(Some("jayme".to_string()), None).await.unwrap();
    ///
    /// println!("{:#?}", notifications);
    /// ```
    pub async fn get_notifications(
        &self,
        username: Option<String>,
        address: Option<String>,
        page: i64,
    ) -> Result<NotifRoot, Box<dyn std::error::Error>> {
        let mut res_address: String = "".to_string();
        if username.is_some() {
            let profile = self.get_user_by_username(&username.unwrap()).await?;
            res_address = profile.result.user.address;
        } else if address.is_some() {
            res_address = address.unwrap();
        } else {
            return Err(Box::from(
                "Please provide a username or address".to_string(),
            ));
        }

        let request_address = format!(
            "https://api.farcaster.xyz/v1/notifications?address={}",
            res_address
        );

        let req = reqwest::get(request_address).await?.text().await?;
        let notifications: NotifRoot = serde_json::from_str(&req)?;

        if page > 0 {
            let mut loopy = 0;
            let mut new_notifications: String = "".to_string();
            while loopy != page {
                new_notifications = reqwest::get(notifications.meta.next.clone())
                    .await?
                    .text()
                    .await?;
                loopy += 1;
            }

            let notifications: NotifRoot = serde_json::from_str(&new_notifications)?;

            return Ok(notifications);
        } else {
            return Ok(notifications);
        }
    }
}
