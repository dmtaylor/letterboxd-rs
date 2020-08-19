use crate::error::Result;
use crate::client;
use crate::defs;
use crate::film;

impl client::Client {
    /// A cursored window over the list of films.
    ///
    /// Use the ‘next’ cursor to move through the list. The response will include the film
    /// relationships for the signed-in member and the member indicated by the member LID if
    /// specified.
    pub async fn films(&self, request: &film::defs::FilmsRequest) -> Result<film::defs::FilmsResponse> {
        self.get_with_query("films", request).await
    }

    /// Get a list of services supported by the /films endpoint.
    ///
    /// Services are returned in alphabetical order. Some services are only available to paying
    /// members, so results will vary based on the authenticated member’s status.
    pub async fn film_services(&self) -> Result<film::defs::FilmServicesResponse> {
        self.get("films/film-services").await
    }

    /// Get a list of genres supported by the `films` function.
    ///
    /// Genres are returned in alphabetical order.
    pub async fn film_genres(&self) -> Result<film::defs::GenresResponse> {
        self.get("films/genres").await
    }

    /// Get details about a film by ID.
    pub async fn film(&self, id: &str) -> Result<film::defs::Film> {
        self.get(&format!("film/{}", id)).await
    }

    /// Get availability data about a film by ID.
    pub async fn film_availability(&self, id: &str) -> Result<film::defs::FilmAvailabilityResponse> {
        self.get(&format!("film/{}/availability", id)).await
    }

    /// Get details of the authenticated member’s relationship with a film by ID.
    pub async fn film_relationship(&self, id: &str) -> Result<film::defs::FilmAvailabilityResponse> {
        self.get(&format!("film/{}/me", id)).await
    }

    /// Update the authenticated member’s relationship with a film by ID.
    pub async fn update_film_relationship(
        &self,
        id: &str,
        request: &film::defs::FilmRelationshipUpdateRequest,
    ) -> Result<film::defs::FilmRelationshipUpdateResponse> {
        self.patch(&format!("film/{}/me", id), request).await
    }

    /// Get details of the authenticated member’s relationship with a film by ID.
    pub async fn film_relationship_members(
        &self,
        id: &str,
        request: &defs::MemberFilmRelationshipsRequest,
    ) -> Result<defs::MemberFilmRelationshipsResponse> {
        self.get_with_query(&format!("film/{}/members", id), request)
            .await
    }

    //     /film/{id}/report

    /// Get statistical data about a film by ID.
    pub async fn film_statistics(&self, id: &str) -> Result<film::defs::FilmStatistics> {
        self.get(&format!("film/{}/statistics", id)).await
    }
}
