use diesel::prelude::*;
use fake::{rand::{rngs::SmallRng, SeedableRng}, Fake, Faker};

use around::schema::users::dsl;
use around::users::User;

fn main() {
    let mut rng = SmallRng::seed_from_u64(0);
    let users = Faker.fake_with_rng::<[User; 1024], SmallRng>(&mut rng);

    let url = std::env::var("DATABASE_URL").unwrap();
    let mut connection = PgConnection::establish(&url).unwrap();

    diesel::insert_into(dsl::users)
        .values(users)
        .on_conflict_do_nothing()
        .execute(&mut connection)
        .unwrap();
}
