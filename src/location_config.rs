// location_config.rs

pub struct Location {
    pub location_tag: &'static str,
    pub activity_tag: &'static str,
    pub venue_tags: &'static [&'static str],
}

pub static LOCATIONS: &[Location] = &[
    Location {
        /// Give it a memorable name like, 'Near London Office', 'Close to Home' etc
        location_tag: "Near Holborn",
        /// What sport are we searching for? You can find this on the booking page
        /// https://bookings.better.org.uk/location/britannia-leisure-centre/squash-court-40min/2024-03-26/by-time
        /// badminton-court-40min - squash-court-40min
        activity_tag: "badminton-40min",
        /// Better venue tags go here, you can find them by visiting your venues activity booking page like this
        /// https://bookings.better.org.uk/location/finsbury-leisure-centre <- We want the end bit
        venue_tags: &["finsbury-leisure-centre", "britannia-leisure-centre", "whitechapel-sports-centre"],
    },
    Location {
        location_tag: "Near Holborn",
        activity_tag: "squash-court-40min",
        venue_tags: &["finsbury-leisure-centre", "britannia-leisure-centre"],
    },
];