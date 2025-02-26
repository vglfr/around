use diesel::prelude::*;
use fake::{rand::{rngs::SmallRng, SeedableRng}, Fake, Faker};

use around::events::Event;
use around::schema::{events, users};
use around::users::User;

fn main() {
    let mut rng = SmallRng::seed_from_u64(0);
    let users = Faker.fake_with_rng::<[User; 1024], SmallRng>(&mut rng);
    let events = Faker.fake_with_rng::<[Event; 8192], SmallRng>(&mut rng);

    let url = std::env::var("DATABASE_URL").unwrap();
    let mut connection = PgConnection::establish(&url).unwrap();

    diesel::insert_into(users::dsl::users)
        .values(users)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .unwrap();

    diesel::insert_into(events::dsl::events)
        .values(events)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .unwrap();
}
