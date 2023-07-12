pub mod db {
    use base::schema::{devices, users, user_devices};

    use chrono::NaiveDateTime;
    use diesel::{prelude::*, PgConnection};

    use crate::models::UserDevice;

    enum IdOrBuild<T> {
        Id(i32),
        Build(T),
    }

    pub struct UserDeviceBuilder<'a> {
        user: IdOrBuild<&'a str>,
        device: IdOrBuild<&'a str>,
    }

    impl<'a> Default for UserDeviceBuilder<'a> {
        fn default() -> Self {
            UserDeviceBuilder { user: IdOrBuild::Build("email@email.com"), device : IdOrBuild::Build("pubkey") }
        }
    }

    impl<'a> UserDeviceBuilder<'a> {
        pub fn build(self, conn: &mut PgConnection) -> Result<UserDevice, diesel::result::Error> {
            let user_id = match self.user {
                IdOrBuild::Id(id) => id,
                IdOrBuild::Build(email) => {
                    diesel::insert_into(users::table)
                        .values(users::email.eq(email))
                        .get_result::<(i32, String, NaiveDateTime)>(conn)?
                        .0
                },
            };

            let device_id = match self.device {
                IdOrBuild::Id(id) => id,
                IdOrBuild::Build(pubkey) => {
                    diesel::insert_into(devices::table)
                        .values((devices::pubkey.eq(pubkey), devices::password.eq("password")))
                        .get_result::<(i32, String, String, NaiveDateTime)>(conn)?
                        .0
                }
            };

            diesel::insert_into(user_devices::table)
                .values((user_devices::user_id.eq(user_id), user_devices::device_id.eq(device_id)))
                .get_result::<(i32, i32, NaiveDateTime)>(conn)
                .map(|row| UserDevice { user_id: row.0, device_id: row.1 })
        }

        pub fn device_id(mut self, id: i32) -> Self {
            self.device = IdOrBuild::Id(id);
            self
        }

        pub fn pubkey(mut self, pubkey: &'a str) -> Self {
            self.device = IdOrBuild::Build(pubkey);
            self
        }

        pub fn user_id(mut self, id: i32) -> Self {
            self.user = IdOrBuild::Id(id);
            self
        }

        pub fn email(mut self, email: &'a str) -> Self {
            self.user = IdOrBuild::Build(email);
            self
        }
    }
}
