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
        delLoading: boolean;
        index: number;
        window: any;
        autoRun: boolean;
        loading:boolean;
        constructor({ size, runningFlag, autoRun }) {
            this.size = size;

            this.tasks = [] as any[];

            this.delLoading = false;

            this.loading = true;

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
            if (this.isRunning()) {
                return;
            }

            this.runningFlag = true;

            let i = this.size;
            this.window.emit("seek_status", this.runningFlag);
            while (i--) {
                this.processTask();
            }
        }

        stop() {
            this.runningFlag = false;
            this.delLoading=true;
            setInterval(()=>{
               if(!this.isRunning()){
                this.window.emit("seek_status", this.runningFlag);
                this.delLoading=false;
               }
            },200)
            
        }

        addTasks(tasks: any[]) {
            console.log(this.tasks)
            Array.prototype.push.apply(this.tasks, tasks);
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

        removeTask(index: number) {
            this.index--;
            this.tasks.splice(index, 1);
        }

        removeTaskByid(id) {

        }

        processTask() {
            if (!this.runningFlag) {
                this.stop();
            } else {
                const task: any = this.tasks[this.index];
                console.log("当前池剩余数量：" + this.tasks.length + ",当前下标为" + this.index,",当前正在运行数量为" + this.runningProcessorCount);
                if (task) {
                    this.runningProcessorCount++;
                    this.index++;
                    if (task.status != 0) {
                        this.runningProcessorCount--;
                        this.processTask();
                    } else {
                        task.status = 3;
                        invoke("retrieve_data", { retrievingSmov: task }).then((res: any) => {
                            if (res.code == 200) {
                                task.status = 1;
                            } else {
                                task.status = 2;
                            }
                        }).finally(() => {
                            this.runningProcessorCount--;
                            this.processTask();
                        });
                    }
                }
                else {
                    setTimeout(() => {
                        if (this.tasks.length === this.index && this.runningProcessorCount === 0) {
                            this.stop();
                        } else {
                            this.processTask();
                        }
                    }, 500);

                }
            }


        }
    }
}