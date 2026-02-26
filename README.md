# Dynamic Batching Service 

A robust, multi-threaded request processing system built with **Rust**, focusing on safe concurrency, workload management, and synchronized communication.

## 🎯 Program Objectives (Goal)
The primary goal of this service is to manage incoming data requests in a **thread-safe environment**. It simulates a high-load "Batching Service" where:
- Requests are handled in parallel using worker threads.
- System resources are protected by a maximum queue size to prevent overloading.
- Timeouts are enforced to ensure no request hangs indefinitely.
- Final results are synchronized and returned to the primary process for display.

## ⚙️ Core Logic & Requirements
The service operates based on a separate thread for each user request. The system follows these strict operational rules:

1.  **Time-Based Processing**: Heavy processing begins its task periodically based on a predefined time interval.
2.  **Capacity-Based Trigger**: If the number of requests in the queue reaches the **Maximum Queue Size**, the heavy processing starts immediately, regardless of the time period, and a new cycle begins.
3.  **Timeout Management**: Each request has a **Maximum Waiting Time**. If this duration is exceeded before a response is ready, the corresponding thread returns a "Timeout" response to the user.

## 🔑 Key Variable Definitions
- **`Request`**: A struct representing the input data, containing an `id` (u32) and a `data` string.
- **`Response`**: A struct representing the processed result, used to track the outcome of each request.
- **`RequestQueue`**: A centralized buffer that holds incoming requests until the processing threshold is met.
- **`ResponseQueue`**: A thread-safe storage (wrapped in `Arc<Mutex>`) that holds the results along with an `Instant` timestamp for timeout monitoring.
- **`heavy_processing_period`**: A user-defined duration that simulates the time taken by a complex computation (e.g., AI inference).
- **`max_response_wait_time`**: The maximum duration a request is allowed to stay in the system before timing out.

## 🛠️ Function Definitions
- **`RequestQueue::enqueue`**: Attempts to add a request to the queue; returns `false` if the queue is at maximum capacity.
- **`ResponseQueue::dequeue`**: A blocking loop that waits for a response to appear in the queue or returns `None` if the `timeout` is reached.
- **`heavy_processing()`**: Simulates a resource-intensive task using `thread::sleep` and appends a processing tag to the data.
- **`handle_request()`**: Manages the life-cycle of a single request, including locking the `ResponseQueue` and waiting for the result.
- **`main()`**: The entry point that initializes parameters, spawns worker threads, and manages the **MPSC (Multi-Producer, Single-Consumer)** channel to aggregate final outputs.

## 📤 Program Output
1. **User Prompts:** Asks for `heavy processing period`, `max queue size`, and `max wait time`.
2. **Real-time Logs:** Displays processing status (e.g., "Request #0 in heavy processing").
3. **Error Handling:** Prints `Timeout` if a thread fails to receive an answer within the limit.
4. **Final Result:** The main thread collects all processed responses from the channel and prints them to the console.

