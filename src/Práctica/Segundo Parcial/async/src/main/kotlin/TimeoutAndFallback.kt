package org.austral

import kotlinx.coroutines.TimeoutCancellationException
import kotlinx.coroutines.delay
import kotlinx.coroutines.withTimeout
import kotlin.system.measureTimeMillis

suspend fun main() {
    var time = measureTimeMillis { println(getUserProfile(1)) }
    println("Measured time: $time")
    time = measureTimeMillis { println(getUserProfile(1, 1000L)) }
    println("Measured time: $time")
}

suspend fun getUserProfile(userId: Int, datasourceDelay: Long = 5000L): String {
    return try {
        withTimeout(2000L) { getUserFromDatasource(userId, datasourceDelay) }
    } catch (_: TimeoutCancellationException) {
        "Default user"
    }
}

private suspend fun getUserFromDatasource(userId: Int, datasourceDelay: Long): String {
    delay(datasourceDelay); return "Profile of user n° $userId"
}