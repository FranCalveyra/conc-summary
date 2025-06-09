package org.austral

import kotlinx.coroutines.async
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import kotlin.system.measureTimeMillis

fun main() {
    val time = measureTimeMillis {
        println(fetchDataFromSources())
    }
    println("Total time: $time") // Es aproximadamente 3 segundos
}

fun fetchDataFromSources(): List<String> = runBlocking {
    val firstDatasource = async { getDataFromFirstDatasource() }
    val secondDatasource = async { getDataFromSecondDatasource() }
    val thirdDatasource = async { getDataFromThirdDatasource() }
    return@runBlocking firstDatasource.await() + secondDatasource.await() + thirdDatasource.await()
}

private suspend fun getDataFromFirstDatasource(): List<String> {
    delay(3000L); return listOf("A", "B")
}

private suspend fun getDataFromSecondDatasource(): List<String> {
    delay(3000L)
    return listOf("C")
}

private suspend fun getDataFromThirdDatasource(): List<String> {
    delay(3000L)
    return listOf("D", "E")
}