pub mod db {
    use base::schema::{devices, users};

    use chrono::NaiveDateTime;
    use diesel::{prelude::*, PgConnection};

    use crate::models::Device;

    #[derive(Default)]
    pub struct DeviceBuilder<'a> {
        user_id: Option<i32>,
        email: Option<&'a str>,
    }

    impl<'a> DeviceBuilder<'a> {
        pub fn build(self, conn: &mut PgConnection) -> Result<Device, diesel::result::Error> {
            let user_id = if let Some(user_id) = self.user_id {
                user_id
            } else {
                diesel::insert_into(users::table)
                    .values(users::email.eq(self.email.unwrap_or("email@email.com")))
                    .get_result::<(i32, String, NaiveDateTime)>(conn)?
                    .0
            };

            diesel::insert_into(devices::table)
                .values((devices::user_id.eq(user_id), devices::pubkey.eq("pubkey"), devices::password.eq("password")))
                .get_result::<(i32, i32, String, String, NaiveDateTime)>(conn)
                .map(|row| Device { id: row.0, user_id: row.1, pubkey: row.2, created_at: row.4 })

        }

        pub fn user_id(mut self, id: i32) -> Self {
            self.user_id = Some(id);
            self
        }

        pub fn email(mut self, email: &'a str) -> Self {
            self.email = Some(email);
            self
        }
    }
}
