
export module ThreadPool {
    export class Task {
        callback: (any) => void;
        params: any;
        processor: (any) => Promise<unknown>;
        constructor({ params, processor, callback }) {
            this.params = params;
            if (typeof processor === 'function') {
                this.processor = processor;
            }
            else {
                throw new Error('processor must be a funtion');
            }

            if (typeof callback === 'function') {
                this.callback = callback || (() => { });
            }
            else {
                throw new Error('callback must be a funtion');
            }
        }
    }

    // 异步池
    export class FixedThreadPool {
        runningFlag: boolean;
        runningProcessorCount: number;
        tasks: any[];
        size: any;
        constructor({
            size,
            tasks
        }) {
            this.size = size;

            this.tasks = [];
            this.addTask(...tasks);

            this.runningFlag = false;
            this.runningProcessorCount = 0;
        }

        isRunning() {
            return this.runningFlag || this.runningProcessorCount > 0;
        }

        start() {
            if (this.isRunning()) {
                return;
            }

            this.runningFlag = true;

            let i = this.size;
            while (i--) {
                this.processTask();
                this.runningProcessorCount++;
            }
        }

        stop() {
            this.runningFlag = false;
        }

        addTask(...tasks) {
            tasks.forEach(task => {
                if (task instanceof Task) {
                    this.tasks.push(task);
                }
                else {
                    console.error('expected to be instanceof Task');
                }
            })
        }

        processTask() {
            if (!this.runningFlag) {
                this.runningProcessorCount--;
                console.log('stop');
                return;
            }

            const task: any = this.tasks.shift();

            if (task) {
                const prom = task.processor(task.params);
                if (prom instanceof Promise) {
                    let cb;
                    prom.then(data => {
                        cb = task.callback(data);
                    }).finally(() => {
                        if (cb instanceof Promise) {
                            cb.finally(() => {
                                this.processTask();
                            });
                        }
                        else {
                            this.processTask();
                        }
                    });
                }
            }
            else {
                setTimeout(() => {
                    this.processTask();
                }, 500);
            }
        }
    }
}


// class Task {
//     constructor({ params, processor = () => { }, callback = () => { } }) {
//         this.params = params;
//         if (typeof processor === 'function') {
//             this.processor = processor;
//         }
//         else {
//             throw new Error('processor must be a funtion');
//         }

//         if (typeof callback === 'function') {
//             this.callback = callback || (() => { });
//         }
//         else {
//             throw new Error('callback must be a funtion');
//         }
//     }
// }

// // 异步池
// class FixedThreadPool {
//     constructor({
//         size,
//         tasks
//     }) {
//         this.size = size;

//         this.tasks = [];
//         this.addTask(...tasks);

//         this.runningFlag = false;
//         this.runningProcessorCount = 0;
//     }

//     isRunning() {
//         return this.runningFlag || this.runningProcessorCount > 0;
//     }

//     start() {
//         if (this.isRunning()) {
//             return;
//         }

//         this.runningFlag = true;

//         let i = this.size;
//         while (i--) {
//             this.processTask();
//             this.runningProcessorCount++;
//         }
//     }

//     stop() {
//         this.runningFlag = false;
//     }

//     addTask(...tasks) {
//         tasks.forEach(task => {
//             if (task instanceof Task) {
//                 this.tasks.push(task);
//             }
//             else {
//                 console.error('expected to be instanceof Task');
//             }
//         })
//     }

//     processTask() {
//         if (!this.runningFlag) {
//             this.runningProcessorCount--;
//             console.log('stop');
//             return;
//         }

//         const task = this.tasks.shift();

//         if (task) {
//             const prom = task.processor(task.params);
//             if (prom instanceof Promise) {
//                 let cb;
//                 prom.then(data => {
//                     cb = task.callback(data);
//                 }).finally(() => {
//                     if (cb instanceof Promise) {
//                         cb.finally(() => {
//                             this.processTask();
//                         });
//                     }
//                     else {
//                         this.processTask();
//                     }
//                 });
//             }
//         }
//         else {
//             setTimeout(() => {
//                 this.processTask();
//             }, 500);
//         }
//     }
// }


///范例

// <script lang="ts">
// import { defineComponent, reactive, ref } from 'vue'
// import {ThreadPool} from '../ts/ThreadPool'

// export default defineComponent({
//     setup() {
//         // 任务实体

//         function genTaskObj(i) {
//             return new ThreadPool.Task({
//                 params: i,
//                 processor: (params) => {
//                     // console.log("线程"+i+"正在运行");
//                     return new Promise(resolve => {
//                         setTimeout(() => {
//                             resolve(params);
//                         }, 1000);
//                     });
//                 },
//                 callback: (data) => {
//                     console.log(`线程 ${i}, rst is`, data);
//                 }
//             });
//         };



//         const test = () => {
//             // const tasks7 = [
//             //     genTaskObj(1),
//             //     genTaskObj(2),
//             //     genTaskObj(3),
//             // ];

//             let tasks7:any[] = [];

//             for (let i = 1 ; i<100 ; i++){
//                 tasks7.push(
//                     genTaskObj(i)
//                 )
//             }
//             let pool7 = new ThreadPool.FixedThreadPool({
//                 size: 10,
//                 tasks: [...tasks7] //, ...tasks7, ...tasks7
//             })
//             pool7.start();

//             // setTimeout(() => {
//             //     pool7.stop();
//             // }, 3000); // 3s后停止
//         }



//         return {
//             test
//         }
//     }
// })
// </script>