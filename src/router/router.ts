import { createRouter,createWebHistory } from "vue-router";
import Index from '../views/index.vue';
import smovFile from '../views/SmovFile.vue';
import test from '../views/test.vue';
import Setting from '../views/Setting.vue'

const router = createRouter({
    history:createWebHistory(),
    routes:[
        {
            path:'/',
            component:Index,
        },
        {
            path:'/SomvFile',
            component:smovFile,
        },
        {
            path:'/test',
            component:test,
        },
        {
            path:'/setting',
            component:Setting,
        }
    ]
})
export default router;