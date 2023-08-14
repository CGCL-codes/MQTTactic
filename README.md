 ### MQTTactic

  The MQTTactic is our tool for evaluating the security of the MQTT Broker with static analyses. More details and instructions will be uploaded/updated later.




  ### Getting started

  #### 1. Install

  * The MQTTactic works on LLVM IR, So LLVM must be available in your system. Currently, the supported LLVM versions are `llvm-9`, `llvm-10`, `llvm-11`, `llvm-12`, and `llvm-13`.

```
$ sudo apt install gcc-10 g++-10 python3 python3-distutils zlib1g-dev unzip cmake  nodejs ninja-build 
```

* RELEASE
```
$ wget https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04.tar.xz
```


* DEBUG
```
$ wget https://github.com/llvm/llvm-project/releases/download/llvmorg-14.0.0/llvm-project-14.0.0.src.tar.xz
$ tar xvf llvm-project-14.0.0.src.tar.xz && cd llvm-project-14.0.0.src/
$ cmake -S llvm -B build -G Ninja -DCMAKE_BUILD_TYPE=Debug -DLLVM_ENABLE_PROJECTS="clang;lld;llvm;libcxx;libcxxabi"
$ cd build && ninja
```


*   SVF

```
git clone https://github.com/SVF-tools/SVF.git
cd SVF && git checkout 925fb44a
export LLVM_DIR=/root/Document/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04
source ./build.sh
```



```
export MQTT_DIR=/root/Document/mqttactic/MQTTactic
export SVF_DIR=/root/Document/SVF
export SVF_BIN=$SVF_DIR/Release-build
export Z3_DIR=/root/node_modules/z3.obj
export LLVM_DIR=/root/Document/clang+llvm-14.0.0-x86_64-linux-gnu-ubuntu-18.04


export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$LLVM_DIR/lib:$SVF_BIN/svf-llvm/
export PATH=$PATH:$LLVM_DIR/bin:$Z3_DIR/bin:$SVF_BIN/bin
export CPLUS_INCLUDE_PATH=$LLVM_DIR/include:$SVF_DIR/include:$SVF_BIN/include:$Z3_DIR/include:$MQTT_DIR/Include
export C_INCLUDE_PATH=$LLVM_DIR/include:$SVF_DIR/include:$Z3_DIR/include:$Z3_DIR/include:$MQTT_DIR/Include
```







  * Haybale for symbolic execution<br>

```
$ git clone https://github.com/MQTTactic/Haybale
```

*   Cargo

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustup default nightly-2022-08-02
```

*    spin

>$ git clone  https://github.com/nimble-code/Spin.git
>
>$ apt-get install flex bison
>
>$ cd Spin && make -j4
>
>$ cd Src
>
>$ ln -s ​\$(pwd)/spin /usr/bin/spin



*   Other dependencies

```
# Boolector
# Download and build Boolector
$ git clone https://github.com/boolector/boolector
$ cd boolector

# Download and build Lingeling
$ ./contrib/setup-lingeling.sh

# Download and build BTOR2Tools
$ ./contrib/setup-btor2tools.sh

# Build Boolector
$ ./configure.sh --shared && cd build && make && make install
```



```
$ sudo apt-get install zlib1g-dev
$ pip3 install cxxfilt
```




  #### 2. Usage

  * Configuration
      A simple example can be found in `Include/`.



acl_check: acl检查函数

deliver_to_subscribers: 查找订阅者并投递的函数

deliver: 投递的函数，投递给某一个订阅者

sub_add: 新增订阅的函数

sub_remove

```python
# CONFIG.py
config = {
    # handlers
    "handle__connect": "github_0com_1VolantMQ_1volantmq_1connection.Manager.OnConnection",
    "handle__publish": "github_0com_1VolantMQ_1volantmq_1connection.impl.onPublish",
    "handle__pubrel": "github_0com_1VolantMQ_1volantmq_1connection.impl.onAck",
    "handle__subscribe": "github_0com_1VolantMQ_1volantmq_1connection.session.SignalSubscribe",
    "handle__unsubscribe": "github_0com_1VolantMQ_1volantmq_1connection.session.SignalUnSubscribe",
    "handle__disconnect": "github_0com_1VolantMQ_1volantmq_1connection.session.SignalConnectionClose",
    "handle__ACL_revoke": "",
    "send__connack": "",
    "send__puback": "",
    "send__pubrec": "",
    "send__pubcomp": "",
    "send__suback": "",
    "send__unsuback": "",
    "acl_check": "github_0com_1VolantMQ_1volantmq_1auth.TestAuth.ACL",
    "deliver_to_subscribers": "github_0com_1VolantMQ_1volantmq_1topics_1mem.Provider.Publish",
    "deliver": "github_0com_1VolantMQ_1volantmq_1connection.writer.send+github_0com_1VolantMQ_1volantmq_1connection.writer.start+github_0com_1VolantMQ_1volantmq_1connection.writer.signalAndRun",
    "sub_add": "github_0com_1VolantMQ_1volantmq_1subscriber.Type.Subscribe",
    "sub_remove": "github_0com_1VolantMQ_1volantmq_1subscriber.Type.UnSubscribe",
    "acl_revoke": "",

    # authorization
    "acl_check_pattern": '(AccessRead|AccessWrite)',
    "authorization_pub": "AccessWrite",
    "authorization_sub": "AccessRead",
    "authorization_read": "",
    "authorization_store": "",
    "authorization_load": "",
}
```



```cpp
#CONFIG.h

using namespace std;

// handlers
std::string handle__connect = "github_0com_1VolantMQ_1volantmq_1connection.Manager.OnConnection";
std::string handle__publish = "github_0com_1VolantMQ_1volantmq_1connection.impl.onPublish";
std::string handle__pubrel = "github_0com_1VolantMQ_1volantmq_1connection.impl.onAck";
std::string handle__subscribe = "github_0com_1VolantMQ_1volantmq_1connection.session.SignalSubscribe";
std::string handle__unsubscribe = "github_0com_1VolantMQ_1volantmq_1connection.session.SignalUnSubscribe";
std::string handle__disconnect = "github_0com_1VolantMQ_1volantmq_1connection.session.SignalConnectionClose";
std::string handle__ACL_revoke = "";
// handlers_end
std::string send__connack = "";
std::string send__puback = "";
std::string send__pubrec = "";
std::string send__pubcomp = "";
std::string send__suback = "";
std::string send__unsuback = "";
// key operations
std::string acl_check = "github_0com_1VolantMQ_1volantmq_1auth.TestAuth.ACL";
std::string deliver_to_subscribers = "github_0com_1VolantMQ_1volantmq_1topics_1mem.Provider.Publish";
std::string deliver = "github_0com_1VolantMQ_1volantmq_1connection.writer.send+github_0com_1VolantMQ_1volantmq_1connection.writer.start+github_0com_1VolantMQ_1volantmq_1connection.writer.signalAndRun";
std::string sub_add = "github_0com_1VolantMQ_1volantmq_1subscriber.Type.Subscribe";
std::string sub_remove = "github_0com_1VolantMQ_1volantmq_1subscriber.Type.UnSubscribe";
std::string acl_revoke = "";
```





  * CFG analysis

  ```
  $ cd {MQTTactic}/CFGPass/src
  $ make config && make
  $ opt -load ./CFGPass.so -CFG ../complete.bc -enable-new-pm=0 -o /dev/null 2> ../OUTPUT/xxx.output
  ```

  * Symbolic Execution

  ```
  $ cd SymbolicExecution/ && cargo build
  $ cd target/debug/
  $ ./SE "handle__pubrel" "{config_handle__pubrel}" "{type_num}" "{MQTTactic}/CFGPass/" "{LLVM_bitcode_dir}" > ModelCheck/SymbolicExecutionResults/handle__pubrel/Type-{type_num}.log 2>&1
  ```

```
e.g.,

./SE "handle__pubrel" "_ZN10MqttPacket12handlePubRelEv" "0" "{MQTTactic}/CFGPass/" "{MQTTactic}/CFGPass/complete.bc" > {MQTTactic}/ModelCheck/SymbolicExecutionResults/handle__pubrel/Type-0.log 2>&1
```



  * Model Checking

  ```
  spin -a ConcreteModel.pml
  mkdir build
  gcc -DMEMLIM=16384 -DVECTORSZ=4096 -O2 -DXUSAFE -DSAFETY -DNOCLAIM -DBITSTATE -w -o pan ../pan.c
  ./pan -m40000 -E -c0 -e -n  > result.txt
  ```









##### CFG预处理

```
# VarAnalysisPass.cpp

$ cd src/
$ clang -fPIC -fno-rtti -Wno-deprecated -o PTA.cpp.o -c ./PTA.cpp
$ clang  -Wl,-znodelete -fno-rtti -fPIC -shared VarAnalysisPass.cpp -o  ../bin/VarAnalysisPass.so /home/szx/Documents/tools/SVF/Release-build/lib/libSvf.a  PTA.cpp.o
$ cd ../bin/
$ opt -load ./VarAnalysisPass.so -VarAnalysisPass ../tests/Field.bc -enable-new-pm=0 -o /dev/null


clang  -Wl,-znodelete -fno-rtti -fPIC -shared AllFunctions.cpp -o AllFunctions.so
opt -load ./AllFunctions.so -funcs ./complete.bc -enable-new-pm=0 -o complete.bc 2> ./ALLFunctions


clang  -Wl,-znodelete -fno-rtti -fPIC -shared CFGPass.cpp -o CFGPass.so
opt -load ./CFGPass.so -CFG ./complete.bc -enable-new-pm=0 -o /dev/null 2> OUTPUT/xxx.output
```





##### SymbolicExecution

```
./SE "handle__pubrel" "_ZN10MqttPacket12handlePubRelEv" "0" "/home/szx/Documents/Experiments/FlashMQ/CFGPass/" "/home/szx/Documents/Experiments/FlashMQ/CFGPass/complete.bc" > /home/szx/Documents/Experiments/FlashMQ/ModelCheck/SymbolicExecutionResults/handle__pubrel/Type-0.log 2>&1
```

```
./SE "handle__disconnect" "github_0com_1VolantMQ_1volantmq_1connection.session.SignalConnectionClose" "0" "/home/szx/Documents/Experiments/volantmq/CFGPass/" > /home/szx/Documents/Experiments/volantmq/ModelCheck/SymbolicExecutionResults/handle__disconnect/Type-0.log 2>&1
```

```
./SE "handle__subscribe" "github_0com_1VolantMQ_1volantmq_1connection.session.SignalSubscribe" "0" "/home/szx/Documents/Experiments/volantmq/CFGPass/" > /home/szx/Documents/Experiments/volantmq/ModelCheck/SymbolicExecutionResults/handle__subscribe/Type-0.log 2>&1
```

```
./SE "handle__unsubscribe" "github_0com_1VolantMQ_1volantmq_1connection.session.SignalUnSubscribe" "0" "/home/szx/Documents/Experiments/volantmq/CFGPass/" > /home/szx/Documents/Experiments/volantmq/ModelCheck/SymbolicExecutionResults/handle__unsubscribe/Type-0.log 2>&1
```

```
./SE "handle__publish" "github_0com_1VolantMQ_1volantmq_1connection.impl.onPublish" "0" "/home/szx/Documents/Experiments/volantmq/CFGPass/" > /home/szx/Documents/Experiments/volantmq/ModelCheck/SymbolicExecutionResults/handle__publish/Type-0.log 2>&1
```



*   Type-1_qos0.log
*   Type-1_qos1.log
*   Type-1_qos2.log
*   Type-1_qos0_retained.log



```
./SE "handle__connect" "github_0com_1VolantMQ_1volantmq_1connection.Manager.OnConnection" "0" "/home/szx/Documents/Experiments/volantmq/CFGPass/" > /home/szx/Documents/Experiments/volantmq/ModelCheck/SymbolicExecutionResults/handle__connect/Type-0.log 2>&1
```

*   Type-0_cleanStartF.log
*   Type-0_cleanStartT.log









##### spin

```
spin -a ConcreteModel.pml
mkdir build
gcc -DMEMLIM=16384 -DVECTORSZ=4096 -O2 -DXUSAFE -DSAFETY -DNOCLAIM -DBITSTATE -w -o pan ../pan.c
./pan -m40000 -E -c0 -e -n  > result.txt
```











#### TODO



-   [ ] rust support(并添加论文中新的挑战)

    1.  由于rustc 生成的llvm ir过于复杂包含许多安全保护的代码，尚需确认CFG是否流通正常（函数指针、）
    2.  handler定位是否准确（需要学习,为什么是这种rmqtt::broker::v5::subscribes::{closure}）
    3.  去除无关的llvm ir（目前有1G）

-   [ ] Variable analysis

    目前尚未解决KBB过多的问题






#### 0x04 环境配置

>   在Ubuntu18.04下测试

##### 1. LLVM-9

>   $ sudo apt-get install clang-9 llvm-9 llvm-9-dev llvm-9-tools
>
>
>
>   $ sudo apt install build-essential cmake







##### 2. cargo

>   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh



##### 3. spin

>
>
>$ git clone  https://github.com/nimble-code/Spin.git
>
>$ apt-get install flex bison
>
>$ cd Spin && make -j4
>
>$ cd Src
>
>$ ln -s $(pwd)/spin /usr/bin/spin





##### 4. Dependency

Boolector

```
# Download and build Boolector
git clone https://github.com/boolector/boolector
cd boolector

# Download and build Lingeling
./contrib/setup-lingeling.sh

# Download and build BTOR2Tools
./contrib/setup-btor2tools.sh

# Build Boolector
./configure.sh --shared && cd build && make && make install
```



```
$ sudo apt-get install zlib1g-dev

$ pip3 install cxxfilt
```





##### 5. demo

```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash

export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm

nvm install 12

npm install --global yarn

yarn add antd

yarn add @craco/craco

yarn add craco-less


yarn build

npm install -global serve

serve -s build
```



##### 6. docker demo

```
$ docker pull syncxxx/mqttdemo
```

