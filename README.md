# Dynamic-batching-service
Suppose we have an internet service that creates a separate thread for each request received from the user. During processing, this thread should put its processed information in a queue, which is periodically processed by a heavy processing and puts the corresponding response in another queue. After answering each of the threads that were waiting for the answer, they take their answers from the queue and do their work independently. The following are received as parameters when bringing up the server:

1-The period of time when heavy processing begins its processing.
2-The maximum number of requests that can be queued. If the number of requests exceeds this limit, regardless of the time period of item 1, the requests will be answered by heavy processing and then a new period will start.
3-The maximum waiting time for each request to receive a response, which is definitely a number greater than 1, if this waiting time is exceeded, the corresponding thread will give a timeout response to its user.
