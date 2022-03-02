import { getAll } from '@tauri-apps/api/window';

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
    export class FixedThreadPool {
        runningFlag: any;
        runningProcessorCount: number;
        tasks: any[];
        size: any;
        index: number;
        window: any;
        autoRun: boolean;
        constructor({ size, tasks, runningFlag,autoRun }) {
            this.size = size;

            this.tasks = [];
            this.addTasks(...tasks);

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

        addTasks(...tasks) {
            tasks.forEach(task => {
                if (task instanceof Task) {
                    this.tasks.push(task);
                }
                else {
                    console.error('expected to be instanceof Task');
                }
            })
        }
        addTask(task) {
            if (task instanceof Task) {
                //console.log("程序池中插入");
                console.log("当前池剩余数量：" + this.tasks.length + ",当前下标为" + this.index);
                this.tasks.push(task);
                if (this.tasks.length > this.index && this.autoRun == true && this.runningFlag == false) {
                    //console.log("开始程序");
                    this.start();
                }
            }
            else {
                console.error('expected to be instanceof Task');
            }
        }

        processTask() {
            if (!this.runningFlag) {
                //this.runningProcessorCount--;
                return;
            }
            // console.log(this.index)
            //const task: any = this.tasks.shift();  //这里移除掉了 得修改一下 先修改状态 然后根据检索状态 决定去留 
            const task: any = this.tasks[this.index];   //这里每次都是获取第一位 第一个执行的时候 还没删除呢 需要修改获取方式
            if (task) {
                this.runningProcessorCount++;
                this.index++;
                task.params.status = 1;
                const prom = task.processor(task.params);
                if (prom instanceof Promise) {
                    let cb;
                    prom.then(data => {
                        cb = task.callback(data);
                    }).finally(() => {
                        //this.tasks.shift();
                        //this.tasks.splice(nowIndex,1);  //在这出现了下标的变动
                        this.runningProcessorCount--;
                        task.params.status = 2;
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
                //setTimeout(() => {
                //    this.processTask();
                //}, 500); 
                //当不存在下一项时 停止执行
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