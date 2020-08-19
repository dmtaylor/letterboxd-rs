#![allow(dead_code, missing_docs)]
#![allow(clippy::enum_variant_names, clippy::large_enum_variant)]

use crate::defs::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Film {
    /// The LID of the film.
    pub id: String,
    /// The title of the film.
    pub name: String,
    /// The original title of the film, if it was first released with a
    /// non-English title.
    pub original_name: Option<String>,
    /// The other names by which the film is known (including alternative
    /// titles and/or foreign translations).
    pub alternative_names: Vec<String>,
    /// The year in which the film was first released.
    pub release_year: u16,
    /// The tagline for the film.
    pub tagline: String,
    /// A synopsis of the film.
    pub description: String,
    /// The film’s duration (in minutes).
    pub run_time: u16,
    /// The film’s poster image (2:3 ratio in multiple sizes).
    pub poster: Image,
    /// The film’s backdrop image (16:9 ratio in multiple sizes).
    pub backdrop: Image,
    /// The backdrop’s vertical focal point, expressed as a proportion of the
    /// image’s height, using values between 0.0 and 1.0. Use when cropping the
    /// image into a shorter space, such as in the page for a film on the
    /// Letterboxd site.
    pub backdrop_focal_point: f32,
    /// The film’s trailer.
    pub trailer: FilmTrailer,
    /// The film’s genres.
    pub genres: Vec<Genre>,
    /// The film’s contributors (director, cast and crew) grouped by discipline.
    pub contributions: Vec<FilmContributions>,
    /// A list of relevant URLs to this entity, on Letterboxd and external
    /// sites.
    pub links: Vec<Link>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FilmAutocompleteRequest {
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// The word, partial word or phrase to match against.
    input: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum FilmAvailabilityService {
    Amazon,
    AmazonVideo,
    AmazonPrime,
    #[allow(non_camel_case_types)]
    iTunes,
    Netflix,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmAvailability {
    /// The service.
    pub service: FilmAvailabilityService,
    /// The service’s name.
    pub display_name: String,
    /// The regional store for the service. Not all countries are supported on
    /// all services.
    pub country: Country,
    /// The unique ID (if any) for the film on the store.
    pub id: String,
    /// The fully qualified URL for the film on this store.
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmAvailabilityResponse {
    /// The list of stores where the film is available for streaming or
    /// purchasing, in order of preference. If the member has not specified
    /// their preferred stores for a service, the USA store will be assumed.
    pub items: Option<Vec<FilmAvailability>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FilmContribution {
    /// The type of contribution.
    #[serde(rename = "type")]
    contribution_type: ContributionType,
    /// The film.
    film: FilmSummary,
    /// The name of the character (only when type is Actor).
    character_name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmContributions {
    /// The type of contribution.
    pub contribution_type: Option<ContributionType>,
    /// The list of contributors of the specified type for the film.
    pub contributors: Vec<ContributorSummary>,
}

// TODO: Ordering, Dedup
#[derive(Serialize, Debug, Clone)]
pub enum FilmStatus {
    Released,
    NotReleased,
    InWatchlist,
    NotInWatchlist,
    Watched,
    NotWatched,
    FeatureLength,
    NotFeatureLength,
}

// TODO: Ordering
#[derive(Serialize, Debug, Clone)]
pub enum FilmRelationshipType {
    Watched,
    NotWatched,
    Liked,
    NotLiked,
    InWatchlist,
    NotInWatchlist,
    Favorited,
}

#[derive(Serialize, Debug, Clone)]
enum FilmContributionsSort {
    FilmName,
    ReleaseDateLatestFirst,
    ReleaseDateEarliestFirst,
    RatingHighToLow,
    RatingLowToHigh,
    FilmDurationShortestFirst,
    FilmDurationLongestFirst,
    FilmPopularity,
    FilmPopularityThisWeek,
    FilmPopularityThisMonth,
    FilmPopularityThisYear,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FilmContributionsRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// The order in which the films should be returned. Defaults to
    /// FilmPopularity, which is an all-time measurement of the amount of
    /// activity the film has received. The FilmPopularityWithFriends values
    /// are only available to signed-in members and consider popularity amongst
    /// the signed-in member’s friends.
    sort: FilmContributionsSort,
    /// The type of contribution.
    #[serde(rename = "type")]
    contribution_type: ContributionType,
    /// Specify the LID of a genre to limit films to those within the specified
    /// genre.
    genre: String,
    /// Specify the starting year of a decade (must end in 0) to limit films to
    /// those released during the decade. 1990
    decade: u16,
    /// Specify a year to limit films to those released during that year. 1994
    year: u16,
    /// Specify the ID of a supported service to limit films to those available
    /// from that service. The list of available services can be found by using
    /// the /films/film-services endpoint.
    service: String,
    /// Specify one or more values to limit the list of films accordingly.
    /// where=Watched&where=Released
    #[serde(rename = "where")]
    where_film_status: Vec<FilmStatus>,
    /// Specify the LID of a member to limit the returned films according to
    /// the value set in memberRelationship.
    member: String,
    /// Must be used in conjunction with member. Defaults to Watched. Specify
    /// the type of relationship to limit the returned films accordingly.
    member_relationship: FilmRelationshipType,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns films from the member’s account. Use Only to return films from
    /// the member’s friends, and All to return films from both the member and
    /// their friends.
    include_friends: IncludeFriends,
    /// Specify a tag code to limit the returned films to those tagged
    /// accordingly.
    tag_code: String,
    /// Must be used with tag. Specify the LID of a member to focus the tag
    /// filter on the member.
    tagger: String,
    /// Must be used in conjunction with tagger. Defaults to None, which
    /// filters tags set by the member. Use Only to filter tags set by the
    /// member’s friends, and All to filter tags set by both the member and
    /// their friends.
    include_tagger_friends: IncludeFriends,
}

#[derive(Deserialize, Debug, Clone)]
struct FilmContributionsResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    /// The list of contributions.
    items: Vec<FilmContribution>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmIdentifier {
    /// The LID of the film.
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmRelationship {
    /// Will be true if the member has indicated they’ve seen the film (via the
    /// ‘eye’ icon) or has a log entry for the film.
    pub watched: bool,
    /// Will be true if the member likes the film (via the ‘heart’ icon).
    pub liked: bool,
    /// Will be true if the member listed the film as one of their four
    /// favorites.
    pub favorited: bool,
    /// Will be true if the film is in the member’s watchlist.
    pub in_watchlist: bool,
    /// The member’s rating for the film.
    pub rating: Option<f32>,
    /// A list of LIDs for reviews the member has written for the film in the
    /// order they were added, with most recent reviews first.
    pub reviews: Vec<String>,
    /// A list of LIDs for log entries the member has added for the film in
    /// diary order, with most recent entries first.
    pub diary_entries: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum FilmRelationshipUpdateMessageCode {
    InvalidRatingValue,
    UnableToRemoveWatch,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum FilmRelationshipUpdateMessage {
    Error {
        /// The error message code.
        code: FilmRelationshipUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
}

/// When PATCHing a film relationship, you may send all of the current property
/// struct values, or just those you wish to change. Properties that violate
/// business rules (see watched below) or contain invalid values will be
/// ignored.
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilmRelationshipUpdateRequest {
    /// Set to true to change the film’s status for the authenticated member to
    /// ‘watched’ or false for ‘not watched’. If the status is changed to
    /// ‘watched’ and the film is in the member’s watchlist, it will be removed
    /// as part of this action. You may not change the status of a film to ‘not
    /// watched’ if there is existing activity (a review or diary entry) for
    /// the authenticated member—check the messages returned from this endpoint
    /// to ensure no such business rules have been violated.
    pub watched: Option<bool>,
    /// Set to true to change the film’s status for the authenticated member to
    /// ‘liked’ or false for ‘not liked’.
    pub liked: Option<bool>,
    /// Set to true to add the film to the authenticated member’s watchlist, or
    /// false to remove it.
    pub in_watchlist: Option<bool>,
    /// Accepts values between 0.5 and 5.0, with increments of 0.5, or null (to
    /// remove the rating).
    pub rating: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmRelationshipUpdateResponse {
    /// The response object.
    pub data: FilmRelationship,
    /// A list of messages the API client should show to the user.
    pub messages: Vec<FilmRelationshipUpdateMessage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmServicesResponse {
    // The list of film services.
    pub items: Vec<Service>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmStatistics {
    /// The film for which statistics were requested.
    pub film: FilmIdentifier,
    /// The number of watches, ratings, likes, etc. for the film.
    pub counts: FilmStatisticsCounts,
    /// The weighted average rating of the film between 0.5 and 5.0. Will not
    /// be present if the film has not received sufficient ratings.
    pub rating: Option<f32>,
    /// A summary of the number of ratings at each increment between 0.5 and
    /// 5.0.
    pub ratings_histogram: Vec<RatingsHistogramBar>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmStatisticsCounts {
    /// The number of members who have watched the film.
    pub watches: usize,
    /// The number of members who have liked the film.
    pub likes: usize,
    /// The number of members who have rated the film.
    pub ratings: usize,
    /// The number of members who have the film as one of their four favorites.
    pub fans: usize,
    /// The number of lists in which the film appears.
    pub lists: usize,
    /// The number of reviews for the film.
    pub reviews: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmSummary {
    /// The LID of the film.
    pub id: String,
    /// The title of the film.
    pub name: String,
    /// The original title of the film, if it was first released with a
    /// non-English title.
    pub original_name: Option<String>,
    /// The other names by which the film is known (including alternative
    /// titles and/or foreign translations).
    pub alternative_names: Option<Vec<String>>,
    /// The year in which the film was first released.
    pub release_year: Option<u16>,
    /// The list of directors for the film.
    pub directors: Vec<ContributorSummary>,
    /// The film’s poster image (2:3 ratio in multiple sizes).
    pub poster: Option<Image>,
    /// Relationships to the film for the authenticated member (if any) and
    /// other members where relevant.
    pub relationships: Vec<MemberFilmRelationship>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmTrailer {
    /// The YouTube ID of the trailer. "ICp4g9p_rgo".
    pub id: String,
    /// The YouTube URL for the trailer.
    /// "https://www.youtube.com/watch?v=ICp4g9p_rgo"
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
struct FilmsAutocompleteResponse {
    // The list of films.
    items: Vec<FilmSummary>,
}

#[derive(Serialize, Debug, Clone)]
pub enum FilmRequestSort {
    FilmName,
    ReleaseDateLatestFirst,
    ReleaseDateEarliestFirst,
    RatingHighToLow,
    RatingLowToHigh,
    FilmDurationShortestFirst,
    FilmDurationLongestFirst,
    FilmPopularity,
    FilmPopularityThisWeek,
    FilmPopularityThisMonth,
    FilmPopularityThisYear,
    FilmPopularityWithFriends,
    FilmPopularityWithFriendsThisWeek,
    FilmPopularityWithFriendsThisMonth,
    FilmPopularityWithFriendsThisYear,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilmsRequest {
    /// The pagination cursor.
    pub cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    pub per_page: Option<usize>,
    /// The order in which the films should be returned. Defaults to
    /// FilmPopularity, which is an all-time measurement of the amount of
    /// activity the film has received. The FilmPopularityWithFriends values
    /// are only available to signed-in members and consider popularity amongst
    /// the signed-in member’s friends.
    pub sort: Option<FilmRequestSort>,
    /// Specify the LID of a genre to limit films to those within the specified
    /// genre.
    pub genre: Option<String>,
    /// Specify the starting year of a decade (must end in 0) to limit films to
    /// those released during the decade. 1990
    pub decade: Option<u16>,
    /// Specify a year to limit films to those released during that year. 1994
    pub year: Option<u16>,
    /// Specify the ID of a supported service to limit films to those available
    /// from that service. The list of available services can be found by using
    /// the /films/film-services endpoint.
    pub service: Option<String>,
    /// Specify one or more values to limit the list of films accordingly.
    /// where=Watched&where=Released
    #[serde(rename = "where")]
    pub where_film_status: Vec<FilmStatus>,
    /// Specify the LID of a member to limit the returned films according to
    /// the value set in memberRelationship.
    pub member: Option<String>,
    /// Must be used in conjunction with member. Defaults to Watched. Specify
    /// the type of relationship to limit the returned films accordingly.
    pub member_relationship: Option<FilmRelationshipType>,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns films from the member’s account. Use Only to return films from
    /// the member’s friends, and All to return films from both the member and
    /// their friends.
    pub include_friends: Option<IncludeFriends>,
    /// Specify a tag code to limit the returned films to those tagged
    /// accordingly.
    pub tag_code: Option<String>,
    /// Must be used with tag. Specify the LID of a member to focus the tag
    /// filter on the member.
    pub tagger: Option<String>,
    /// Must be used in conjunction with tagger. Defaults to None, which
    /// filters tags set by the member. Use Only to filter tags set by the
    /// member’s friends, and All to filter tags set by both the member and
    /// their friends.
    pub include_tagger_friends: Option<IncludeFriends>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmsResponse {
    /// The cursor to the next page of results.
    pub next: Option<Cursor>,
    /// The list of films.
    pub items: Vec<FilmSummary>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Genre {
    /// The LID of the genre.
    pub id: String,
    /// The name of the genre.
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GenresResponse {
    /// The list of genres.
    pub items: Vec<Genre>,
}
