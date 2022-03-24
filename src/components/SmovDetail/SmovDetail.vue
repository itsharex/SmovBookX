<template>
    <div class="smovDetail">
        <el-dialog v-model="dialogVisible" width="85%" :before-close="close" :lock-scroll="true">
            <template #title>
                <div class="noteDiv">
                    <p class="note">WebView2存在视频内存泄漏的问题 ， 等待后续更新 增加软件内观看的功能 ,当前内置播放体验差 建议不要用</p>
                    <p class="note">(我根本没开放 嘿嗨嘿)</p>
                </div>
            </template>
            <!-- <Artplayer @get-instance="getInstance" :option="option" :style="style" /> -->
            <div class="imgDeatil">
                <div class="imgDiv">
                    <el-carousel :interval="0" arrow="hover">
                        <el-carousel-item>
                            <img
                                class="mainImg"
                                :src="data.main_img == '' ? nonePic : convertFileSrc(data.main_img)"
                            />
                        </el-carousel-item>
                        <el-carousel-item v-for="item in data.detail_img" :key="item">
                            <img
                                class="mainImg"
                                :src="convertFileSrc(item)"
                            />
                        </el-carousel-item>
                    </el-carousel>

                    <!-- 本来打算做的图片选择框 -->
                    <!-- <div class="imgTray">
                        <div class="imgTrayItem">
                            <img
                                class="mainImg"
                                :src="data.main_img == '' ? nonePic : convertFileSrc(data.main_img)"
                            />
                        </div>
                    </div>-->
                    <p class="smovTitle">{{ data.title }}</p>
                </div>
                <div class="detail">
                    <!-- <p class="title">详情</p> -->
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
                        <span class="value">{{ data.is_ch ? '是' : '否' }}</span>
                    </p>
                </div>
            </div>

            <template #footer>
                <el-button type="info" color="#C7415B" @click="close">关闭</el-button>

                <el-button type="info" color="#C7415B" @click="toOpen">由本地播放器打开</el-button>
            </template>
        </el-dialog>
    </div>
</template>

<script lang='ts'>
import { shell } from '@tauri-apps/api';
import { join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import { defineComponent } from 'vue';
import nonePic from './NoneImages.png'
import { ElMessage } from 'element-plus';

export default defineComponent({
    props: {
        data: {
            type: Object,
            default: {}
        },
        close: {
            type: Function as any
        },
    },
    setup(props) {
        const option = {
            url: convertFileSrc(props.data.path + '//' + props.data.realname + '.' + props.data.extension),
            fullscreen: true,
            fullscreenWeb: true,
            theme: '#ffad00',
        };
        const style = {
            width: "600px",
            height: "400px",
            margin: "60px auto 0",
        };

        const getInstance = (art: any) => {
            console.log(art);
        };

        const toOpen = async () => {
            try {
                shell.open(await join(props.data.path, props.data.realname + '.' + props.data.extension));
            } catch (e) {
                console.log(e);
                ElMessage.error('打开出现错误，可能没有设置文件类型的默认打开程序');
            }

        }

        return {
            option,
            style,
            getInstance,
            convertFileSrc,
            toOpen,
            nonePic,
            dialogVisible: true
        };
    }
})

</script>
<style lang='less' scoped>
.note {
    font-weight: 700;
    font-size: 12px;
    text-align: justify;
    text-justify: inter-ideograph;
    margin-top: 2px;
    margin-bottom: 2px;
}

.noteDiv {
    background: rgb(255, 229, 229);
    border: 2px solid rgb(255, 72, 72);
    border-radius: 3px;
}
.imgDiv {
    width: 70%;
    height: 70%;
}

.mainImg {
    display: inline-block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 6px;
}

.smovDetail {
    width: 60%;
    height: 50%;
    z-index: 998;
    position: absolute;
    top: 0;
}
.imgDeatil {
    display: flex;
}

.detail {
    display: flex;
    flex-direction: column;
        margin-left: 20px;
    p {
        text-align: justify;
        text-justify: inter-ideograph;
        margin-top: 4px;
        margin-bottom: 4px;
    }
}

.title {
    font-weight: 700;
    font-size: 30px;
}
.key {
    font-size: 17px;
    font-weight: 600;
}
.value {
    font-size: 17px;
    font-weight: 500;
}
.smovTitle {
    font-size: 14px;
    font-weight: 600;
    text-align: justify;
    text-justify: inter-ideograph;
    width: 90%;
}
</style>

<style>
.domSmovDetail {
    width: 100%;
    height: 100%;
}
</style>