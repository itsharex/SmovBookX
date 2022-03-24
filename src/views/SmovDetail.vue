<template>
    <div class="smovDetail">fuck</div>
</template>

<script lang='ts' >
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { defineComponent, onMounted } from "vue";
import { useRoute } from "vue-router";

export default defineComponent({
    props: {

    },
    setup(props) {

        const msg = ref(false);

        const data = ref({} as any);

        const route = useRoute();

        const id = ref(route.params.Id);

        onMounted(() => {
            getData();
        })

        const getData = () => {
            invoke("get_smov_by_id", { id: parseInt(id.value[0]) }).then((res: any) => {
                if (res.code == 200) {
                   console.log(res)
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
        };
    },
});
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
    height: 80%;
}

.mainImg {
    display: inline-block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: 6px;
}
.detailImg {
    max-width: 100%;
    background-size: cover;
}

.smovDetail {
    width: 60%;
    z-index: 998;
    position: absolute;
    top: 0;
}
.imgDeatil {
    display: flex;
    height: 48vh;
}

.detail {
    display: flex;
    flex-direction: column;
    margin-left: 20px;
    width: 30%;
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
.smovTitle {
    font-size: 14px;
    font-weight: 600;
    text-align: justify;
    text-justify: inter-ideograph;
    width: 90%;
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
</style>

<style>
.domSmovDetail {
    width: 100%;
    height: 100%;
}
</style>