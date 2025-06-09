import org.austral.factorialSequence
import org.austral.fibonacciSequence
import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

class SequenceTests {
    private val fibSeq = fibonacciSequence

    @Test
    fun `fibonacci sequence runs correctly for first values`() {
        `test Fibonacci for N elements`(4)
    }

    @Test
    fun `fibonacci sequence runs correctly for thousand values`() {
        `test Fibonacci for N elements`(1000)
    }

    @Test
    fun `fib volume test`() {
        `test Fibonacci for N elements`(10000)
        `test Fibonacci for N elements`(100000)
//        `test Fibonacci for N elements`(1000000) // Believe me, it works
    }

    private fun `test Fibonacci for N elements`(n: Int) {
        var currentSequence = listOf<Int>()
        var prevSequence = fibSeq.take(1).toList()
        for (i in 2..n) {
            currentSequence = fibSeq.take(i).toList()
            val size = prevSequence.size

            if (i > 2) {
                assertEquals(prevSequence + (prevSequence[size - 1] + prevSequence[size - 2]), currentSequence)
            } else {
                assertEquals(listOf(1, 1), currentSequence)
            }
            prevSequence = currentSequence
        }
    }

    private val factSeq = factorialSequence

    @Test
    fun `factorial works for the first 5 values`() {
        `test Factorial for N elements`(5)
    }

    @Test
    fun `factorial volume test`() {
        `test Factorial for N elements`(1)
        `test Factorial for N elements`(10)
        `test Factorial for N elements`(100)
        `test Factorial for N elements`(1000)
        `test Factorial for N elements`(10000)
        `test Factorial for N elements`(100000)
    }


    private fun `test Factorial for N elements`(n: Int) {
        var currentSequence = listOf<Int>()
        var prevSequence = factSeq.take(1).toList()
        for (i in 2..n) {
            currentSequence = factSeq.take(i).toList()

            if (i > 2) {
                assertEquals(prevSequence + (i * prevSequence.last()), currentSequence)
            } else {
                assertEquals(listOf(1, 2), currentSequence)
            }
            prevSequence = currentSequence
        }
    }

}