<template>
    <div>
        <el-button v-if="show" type="primary" @click="show = !show">Primary</el-button>
        <el-button v-if="show" type="success" @click="show = !show">Success</el-button>
    </div>
</template>

<script lang='ts' setup>
let show = false;
// 初始化与绑定监听事件方法 直接竖着给他转个九十度
const scrollInit = () => {
    // 获取要绑定事件的元素
    const scrollDiv = document.getElementById("ImgTray")!;
    // document.addEventListener('DOMMouseScroll', handler, false)
    // 添加滚轮滚动监听事件，一般是用下面的方法，上面的是火狐的写法
    scrollDiv.addEventListener('mousewheel', handler, false)
    // 滚动事件的出来函数
    function handler(event) {
        event.preventDefault();
        // 获取滚动方向
        const detail = event.wheelDelta || event.detail;
        // 定义滚动方向，其实也可以在赋值的时候写
        const moveForwardStep = 1;
        const moveBackStep = -1;
        let flag = 0;
        // 定义滚动距离
        let step = 0;
        // 判断滚动方向,这里的100可以改，代表滚动幅度，也就是说滚动幅度是自定义的
        if (detail < 0) {
            step = moveForwardStep * 70;
            flag = 1;
        } else {
            step = moveBackStep * 70;
            flag = -1;
        }
        // 对需要滚动的元素进行滚动操作

        const end = scrollDiv.scrollLeft + step;

        const max = scrollDiv.scrollWidth - scrollDiv.clientWidth;

        const stepOne = flag * 2;

        const inter = setInterval(() => {
            console.log(scrollDiv.scrollLeft)
            if (scrollDiv.scrollLeft == end || (scrollDiv.scrollLeft == max && flag == 1) || (scrollDiv.scrollLeft == 0 && flag == -1)) {
                window.clearInterval(inter);
            } else {
                scrollDiv.scrollLeft += stepOne;
            }
        }, 1);


    }
}
</script>
<style lang='less' scoped>
</style>