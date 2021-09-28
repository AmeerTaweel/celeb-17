package taweel.ameer.models

import kotlinx.serialization.Required
import kotlinx.serialization.Serializable

@Serializable
data class Artist(
    @Required val id: Int,
    @Required val name: String,
    @Required val alias: String
)
