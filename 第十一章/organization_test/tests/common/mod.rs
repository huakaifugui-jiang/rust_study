/*
 * @Author: wlj
 * @Date: 2022-12-15 14:48:57
 * @LastEditors: wlj
 * @LastEditTime: 2022-12-15 14:51:41
 * @Description: 一些通用的测试函数
 */

 //如果再次运行测试，将会在测试结果中看到一个新的对应 common.rs 文件的测试结果部分，即便这个文件并没有包含任何测试函数，也没有任何地方调用了 setup 函数：
 //我们并不想要common 出现在测试结果中显示 running 0 tests 。我们只是希望其能被其他多个集成测试文件中调用罢了。
// 为了不让 common 出现在测试输出中，我们将创建 tests/common/mod.rs ，而不是创建 tests/common.rs
pub fn setup() {
    // setup code specific to your library's tests would go here
}