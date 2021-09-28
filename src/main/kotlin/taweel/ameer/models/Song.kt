package taweel.ameer.models

import kotlinx.serialization.Required
import kotlinx.serialization.Serializable

@Serializable
data class Song(
    @Required val id: Int,
    @Required val name: String,
    @Required val artistId: Int,
    @Required val tags: List<String> = emptyList(),
    val versionOf: Int = -1,
    val featuredArtists: List<String> = emptyList()
)
