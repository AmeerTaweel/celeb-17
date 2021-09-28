package taweel.ameer.models

import kotlinx.serialization.Required
import kotlinx.serialization.Serializable

@Serializable
data class Config(
    @Required val musicDirectory: String = "~/Music"
)