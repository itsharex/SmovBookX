import { createRouter, createWebHistory } from "vue-router";
import Index from '../views/index.vue';
import smovFile from '../views/SmovFile.vue';
import test from '../views/test.vue';
import Setting from '../views/Setting.vue';
import Main from '../views/Main.vue';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            component: Main,
            redirect: '/index',
            children: [
                {
                    path: '/index',
                    component: Index,
                },
                {
                    path: '/SomvFile',
                    component: smovFile,
                },
                {
                    path: '/test',
                    component: test,
                    redirect: '/test1',
                    children: [
                        {
                            path: '/test1',
                            component: () => import("../views/test1.vue")
                        },
                        {
                            path: '/test2',
                            component: () => import("../views/test2.vue")
                        }
                    ]
                },
                {
                    path: '/setting',
                    component: Setting,
                }
            ]

        },
        {
            path: '/test3',
            component: () => import("../views/test3.vue")
        },
    ]
})
export default router;