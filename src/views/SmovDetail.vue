<template>
    <el-container class="smovDetail">
        <el-header class="editBar" data-tauri-drag-region>
            <!-- <el-button type="primary" @click="detailShow = !detailShow">Primary</el-button> -->
        </el-header>
        <el-main class="ImgShow">
            <img
                class="showImg"
                :src="focus === -1 ? (data.main_img != undefined ? convertFileSrc(data.main_img) : nonePic) : convertFileSrc(data.detail_img[focus])"
            />
        </el-main>

        <div class="ImgTray" id="ImgTray">
            <div
                class="imgLabel"
                :key="-1"
                :class="focus == -1 ? 'imgLabelFocus' : ''"
                @click="focus = -1"
            >
                <img :src="data.main_img != undefined ? convertFileSrc(data.main_img) : nonePic" />
            </div>
            <div
                class="imgLabel"
                v-for="(item, index) in data.detail_img"
                :key="index"
                :class="focus == index ? 'imgLabelFocus' : ''"
                @click="focus = index"
            >
                <img :src="convertFileSrc(item)" />
            </div>
        </div>


        <el-drawer
            v-model="detailShow"
            :with-header="false"
            :modal="true"
            modal-class="detailModal"
        >
            <div class="detail">
                <p>
                    <span class="key">名称：</span>
                    <span class="value">{{ data.name }}</span>
                </p>
                <p>
                    <span class="key">发行日期：</span>
                    <span class="value">{{ data.release_time }}</span>
                </p>
                <p>
                    <span class="key">文件大小：</span>
                    <span class="value">{{ ~~(data.len / 1024 / 1024) }}MB</span>
                </p>
                <p>
                    <span class="key">制作：</span>
                    <span class="value">{{ data.maker }}</span>
                </p>
                <p v-if="data.serie != '无系列'">
                    <span class="key">系列：</span>
                    <span class="value">{{ data.serie }}</span>
                </p>
                <p>
                    <span class="key">文件类型：</span>
                    <span class="value">{{ data.extension }}</span>
                </p>
                <p>
                    <span class="key">导演：</span>
                    <span class="value">{{ data.director }}</span>
                </p>
                <p>
                    <span class="key">是否中文：</span>
                    <span class="value">{{ data.is_ch ? "是" : "否" }}</span>
                </p>
                <p>
                    <span class="key">演员：</span>
                    <span
                        v-for="(item, index) in data.actors"
                        class="value"
                        :key="index"
                        style="margin-right: 2px"
                    >
                        <a href="#">{{ item.name }}</a>
                    </span>
                </p>

                <div class="tags">
                    <span class="key">标签：</span>
                    <span class="tag" v-for="tag in data.tags" :key="tag">
                        <a href="#">{{ tag.name }}</a>
                    </span>
                </div>
            </div>
        </el-drawer>
    </el-container>
</template>

<script lang='ts' >
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { defineComponent, onMounted } from "vue";
import { useRoute } from "vue-router";
import nonePic from "../assets/NoneImages.png";

export default defineComponent({
    props: {

    },
    setup(props) {

        const msg = ref(false);

        const data = ref({} as any);

        const route = useRoute();

        const detailShow = ref(false);

        const id = ref(route.params.Id);

        const focus = ref(-1);

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

        onMounted(() => {
            getData();
            // scrollInit();
        })

        const getData = () => {
            invoke("get_smov_by_id", { id: Number(id.value) }).then((res: any) => {
                if (res.code == 200) {
                    console.log(res.data)
                    data.value = res.data
                }
            }
            ).finally(() => {

            })
        }

        const toOpen = async () => {

        };

        return {
            convertFileSrc,
            toOpen,
            dialogVisible: true,
            msg,
            data,
            detailShow,
            focus,
            nonePic
        };
    },
});
</script>
<style lang='less' scoped>
.smovDetail {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    z-index: 500;
    overflow: hidden;
    border-radius: 8px;
    box-shadow: 10px 10px 5px #888888;
    background-color: #88888821;
}

.detail {
    height: 100%;
    display: flex;
    flex-direction: column;
    p {
        text-align: justify;
        text-justify: inter-ideograph;
        margin-top: 4px;
        margin-bottom: 4px;
    }
}

.key {
    font-size: 15px;
    font-weight: 600;
}
.value {
    font-size: 15px;
    font-weight: 500;
    a {
        color: brown;
        font-weight: 600;
    }
}

.tags {
    display: flex;
    flex-wrap: wrap;
    .tag {
        font-size: 15px;
        margin-right: 5px;
        a {
            color: brown;
            font-weight: 600;
        }
    }
}

.ImgTray {
    background-color: rgba(202, 202, 202, 0.411);
    max-height: 80vw;
    border-radius: 12px;
    position: absolute;
    left: 50%;
    flex-direction: column;
    display: flex;
    padding: 6px;
    overflow: auto;
    transform-origin: left center;
    transform: rotate(270deg) translateX(-45vh);
}



.ImgTray::-webkit-scrollbar {
    display: none; /* Chrome Safari */
}

.imgLabel {
    width: 50px;
    height: 48px;
    padding: 4px;
    background-color: rgba(153, 153, 153, 0.2);
    border: 2px solid rgba(153, 153, 153, 0);
    margin-top: 3px;
    margin-bottom: 3px;
    border-radius: 6px;
    flex-shrink: 0;
    transform: rotate(90deg);
    img {
        width: 100%;
        height: 100%;
        object-fit: cover;
        border-radius: 6px;
    }
}

.imgLabelFocus {
    border: 2px solid rgba(207, 49, 49, 0.712);
}

.ImgShow {
    width: 100vw;
    height: 100vh;
    overflow: hidden;
    display: flex;
    justify-content: center;
    align-items: center;
    img {
        max-width: 100%;
        max-height: 100%;
    }
}

.editBar {
    height: 30px;
}
</style>

<style>
.detailModal {
    background-color: rgba(165, 42, 42, 0);
}

</style>
