# 实现计划

- [x] 1. 创建核心数据结构和类型定义





  - 在 `src/lib.rs` 中创建 `client` 模块
  - 定义 `Command` 枚举，包含 `Get` 和 `Set` 变体，每个变体包含命令参数和 oneshot 响应通道
  - 定义 `Responder<T>` 类型别名为 `oneshot::Sender<mini_redis::Result<T>>`
  - 定义 `RedisClient` 结构体，包含 `mpsc::Sender<Command>` 字段
  - 为 `RedisClient` 实现 `Clone` trait
  - _需求: 1.5, 4.2_

- [ ] 2. 实现连接管理器后台任务
  - 创建 `connection_manager` 异步函数，接受 `mini_redis::client::Client` 和 `mpsc::Receiver<Command>` 作为参数
  - 实现循环接收命令的逻辑，使用 `while let Some(cmd) = rx.recv().await`
  - 为 `Command::Get` 实现处理逻辑：调用 `client.get()` 并通过 oneshot 通道发送响应
  - 为 `Command::Set` 实现处理逻辑：调用 `client.set()` 并通过 oneshot 通道发送响应
  - 确保在通道关闭时函数优雅退出
  - _需求: 1.4, 4.3, 6.2, 6.3_

- [ ] 3. 实现 RedisClient::connect 方法
  - 实现 `pub async fn connect(addr: &str) -> mini_redis::Result<Self>` 方法
  - 使用 `mini_redis::client::connect(addr).await` 建立连接
  - 创建容量为32的 mpsc 通道：`mpsc::channel::<Command>(32)`
  - 使用 `tokio::spawn` 启动 `connection_manager` 后台任务
  - 返回包含发送端的 `RedisClient` 实例
  - _需求: 1.1, 1.2, 1.3_

- [ ] 4. 实现 RedisClient::get 方法
  - 实现 `pub async fn get(&self, key: String) -> mini_redis::Result<Option<Bytes>>` 方法
  - 创建 oneshot 通道用于接收响应
  - 构造 `Command::Get` 并通过 mpsc 通道发送
  - 处理发送失败的情况，返回适当的错误
  - 等待 oneshot 响应并返回结果
  - 处理响应通道关闭的情况
  - _需求: 2.1, 2.2, 2.3, 2.4, 2.5_

- [ ] 5. 实现 RedisClient::set 方法
  - 实现 `pub async fn set(&self, key: String, value: Bytes) -> mini_redis::Result<()>` 方法
  - 创建 oneshot 通道用于接收响应
  - 构造 `Command::Set` 并通过 mpsc 通道发送
  - 处理发送失败的情况，返回适当的错误
  - 等待 oneshot 响应并返回结果
  - 处理响应通道关闭的情况
  - _需求: 3.1, 3.2, 3.3, 3.4, 3.5_

- [ ] 6. 创建示例程序
  - 在 `examples/` 目录创建 `async_client.rs` 示例文件
  - 实现基本的 SET 和 GET 操作示例
  - 实现多任务并发操作示例，展示客户端克隆和并发使用
  - 添加错误处理示例
  - 在 `Cargo.toml` 中配置示例
  - _需求: 1.5, 4.1, 4.2_

- [ ] 7. 编写集成测试
  - 在 `tests/` 目录创建 `integration_test.rs` 文件
  - 编写基本操作测试：测试 SET 后 GET 能获取正确的值
  - 编写并发测试：从多个任务同时发送命令，验证响应正确性
  - 编写错误处理测试：测试连接到无效地址的错误处理
  - 编写资源清理测试：验证客户端丢弃后资源正确清理
  - _需求: 4.4, 4.5, 5.1, 5.2, 5.3, 5.4, 6.1, 6.4, 6.5_
