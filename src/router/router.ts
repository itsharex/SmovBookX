import { createRouter, createWebHistory } from "vue-router";
import Index from '../views/index.vue';
import smovFile from '../views/SmovFile.vue';
import test from '../views/test.vue';
import Setting from '../views/Setting.vue';
import Main from '../views/Main.vue';
import SmovView from '../views/SmovView.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: Main,
            redirect: '/SomvView',
            children: [
                {
                    path: '/index',
                    component: Index,
                    meta: {
                        keepAlive: true,
                        label: "index"
                    }
                },
                {
                    path: '/SomvFile',
                    component: smovFile,
                    meta: {
                        keepAlive: false
                    }
                },
                {
                    path: '/SomvView',
                    component: SmovView,
                    meta: {
                        keepAlive: true
                    }
                },
                {
                    path: '/test',
                    component: test,
                },
                {
                    path: '/setting',
                    component: Setting,
                    meta: {
                        keepAlive: false
                    }
                }
            ]

        },
        {
            name: "seek",
            path: '/seek',
            component: () => import("../views/Seek.vue")
        },
        {
            path: '/SmovDetail/:Id',
            component: () => import("../views/SmovDetail.vue")
        },
    ],

})
export default router;