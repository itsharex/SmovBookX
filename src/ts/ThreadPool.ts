import { invoke } from '@tauri-apps/api';
import { getAll } from '@tauri-apps/api/window';

export module ThreadPool {
    export type task = {
        time: String,
        thread: String,
        msg: String,
        level: String
    }

    export class FixedThreadPool {
        runningFlag: any;
        runningProcessorCount: number;
        tasks: any[];
        size: any;
        loading: boolean;
        index: number;
        window: any;
        autoRun: boolean;
        constructor({ size, runningFlag, autoRun }) {
            this.size = size;

            this.tasks = [] as any[];

            this.loading = false;

            this.runningFlag = runningFlag;
            this.runningProcessorCount = 0;  //正在执行中的线程

            this.index = 0;

            this.window = getAll().filter(val => {
                return val.label === 'main'
            })[0];
            this.autoRun = autoRun;
        }

        isRunning() {
            return this.runningFlag || this.runningProcessorCount != 0;
        }

        start() {
            console.log(this.runningFlag);
            if (this.isRunning()) {
                return;
            }

            this.runningFlag = true;

            let i = this.size;
            this.window.emit("seek_status", this.runningFlag);
            while (i--) {
                this.processTask();
                //this.runningProcessorCount++;
            }
        }

        stop() {
            this.runningFlag = false;
            this.window.emit("seek_status", this.runningFlag);
        }

        addTasks(tasks: any[]) {
            console.log(this.tasks)
            tasks.forEach(task => {
                if (task) {

                    this.tasks.push(task);
                }
                else {
                    console.error('expected to be instanceof Task');
                }
            })
        }
        addTask(task: any) {
            if (task) {
                this.tasks.push(task);
                if (this.tasks.length > this.index && this.autoRun == true && this.runningFlag == false) {
                    this.start();
                }
            }
            else {
                console.error('expected to be instanceof Task');
            }
        }

        removeTask(index) {
            this.index--;
            this.tasks.splice(index, 1);
        }

        removeTaskByid(id) {

        }

        processTask() {
            if (!this.runningFlag) {
                return;
            }

            //获取这套程序的 参数值
            const task: any = this.tasks[this.index];

            if (task) {
                this.runningProcessorCount++;
                this.index++;
                task.status = 3;
                invoke("retrieve_data", { retrievingSmov: task }).then((res: any) => {
                    if (res.code == 200) {
                        task.status = 1;
                    } else {
                        task.status = 2;
                    }
                }).finally(() => {
                    //可以考虑用XEUtil去精准删除 然后 放到其他 队列 
                    this.runningProcessorCount--;
                    //task.params.status = 1;
                    this.processTask();
                });
            }
            else {
                //console.log("当前池剩余数量：" + this.tasks.length + ",当前下标为" + this.index);
                setTimeout(() => {  //延迟500ms防止 炸了
                    if (this.tasks.length == this.index && this.runningProcessorCount == 0) {
                        //当运行到倒数第二个时 因为线程为2 第二次运行直接到了这里 这个时候index为1  池中数量也为1 所以会直接回调停止执行
                        //这里得判断是否还有正在执行的线程
                        this.stop();
                    } else {
                        this.processTask();
                    }
                }, 500);

            }
        }
    }
}