package taweel.ameer.models

import kotlinx.serialization.Required
import kotlinx.serialization.Serializable

@Serializable
data class Library(
    @Required val songs: List<Song> = emptyList(),
    @Required val artists: List<Artist> = emptyList()
)