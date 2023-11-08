<template>
  <view class="content">
    <!-- <image class="logo" src="/static/logo.png" /> -->
    <view class="text-area">
      <text class="title">{{ title }}</text>
    </view>
  </view>
  <view>
    <uni-easyinput
      v-model="input"
      type="text"
      placeholder="md5输入"
      @input="hashcalc"
    />
  </view>
  <view>
    <uni-easyinput v-model="output" type="text" placeholder="" />
  </view>
</template>

<script setup lang="ts">
import { ref } from "vue";
import init, { md5calc, fib,greet ,take_string_by_value} from "wasm";

const input = ref("");
const output = ref("");
const title = ref("rust wasm 测试");

// 参考 https://stackoverflow.com/a/54099484
function array2hex(arrayBuffer: ArrayBuffer)
{
    const buff = new Uint8Array(arrayBuffer);
    const hexOctets = [];
    for (let i = 0; i < buff.length; ++i)
        hexOctets.push(buff[i].toString(16).padStart(2, "0"));

    return hexOctets.join("");
}

// init().then(()=>{
//   greet("wasm");
//   }).catch((err) => {
//       console.error(err);
//     });
init();

const hashcalc = (e) => {
  console.log(typeof e, e);
  const encoder = new TextEncoder();
  // console.log("encoder.encode(e): ", encoder.encode(e));
  let out:ArrayBuffer = md5calc(encoder.encode(e));
  console.log(typeof out, out, array2hex(out));
  output.value = array2hex(out);
};
</script>

<style>
.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.logo {
  height: 200rpx;
  width: 200rpx;
  margin-top: 200rpx;
  margin-left: auto;
  margin-right: auto;
  margin-bottom: 50rpx;
}

.text-area {
  display: flex;
  justify-content: center;
}

.title {
  font-size: 36rpx;
  color: #8f8f94;
}
</style>
