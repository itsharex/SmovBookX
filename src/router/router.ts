import { createRouter,createWebHistory } from "vue-router";
import Index from '../views/index.vue';
import smovFile from '../views/SmovFile.vue';
import test from '../views/test.vue';

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
        }
    ]
})
export default router;