<template>
  <a-modal v-model:open="showModal" :footer="null" :maskClosable="false" @cancel="handleCloseModal">
    <div class="form-container">
      <div class="back" v-if="showCodeLogin">
        <a-button type="link" @click="showCodeLogin = false">返回</a-button>
      </div>
      <div class="title">{{ showCodeLogin ? '短信验证' : '密码登录' }}</div>
      <div class="login-form" v-show="!showCodeLogin">
        <a-form
          :model="state.params"
          name="loginForm"
          size="large"
          :label-col="{ span: 4 }"
          :wrapper-col="{ span: 16 }"
          autocomplete="off"
        >
          <a-form-item
            label="用户名"
            name="username"
            :rules="[{ required: true, message: '请输入用户名!' }]"
          >
            <a-input v-model:value="state.params.username" />
          </a-form-item>
          <a-form-item
            label="密码"
            name="password"
            :rules="[{ required: true, message: '请输入密码!' }]"
          >
            <a-input-password v-model:value="state.params.password"></a-input-password>
          </a-form-item>

          <a-form-item name="remember" :wrapper-col="{ offset: 4, span: 16 }">
            <a-checkbox v-model:checked="state.params.remember">记住密码</a-checkbox>
          </a-form-item>
          <a-form-item :wrapper-col="{ offset: 4, span: 16 }">
            <a-button type="primary" class="login-btn">登录</a-button>
          </a-form-item>
        </a-form>
      </div>
      <div class="code-form" v-show="showCodeLogin">
        <a-form
          :model="state.codeParams"
          name="codeLoginForm"
          size="large"
          :wrapper-col="{ span: 24 }"
          autocomplete="off"
        >
          <a-form-item
            label=""
            name="cardNo"
            :rules="[{ required: true, message: '请输入登录账号绑定的证件号后4位!' }]"
          >
            <a-input
              placeholder="请输入登录账号绑定的证件号后4位"
              :maxlength="4"
              v-model:value="state.codeParams.cardNo"
              @change="handleCardChange"
            />
          </a-form-item>
          <a-form-item label="" name="code" :rules="[{ required: true, message: '请输入验证码!' }]">
            <div class="flex code-container">
              <a-input
                placeholder="输入验证码"
                :maxlength="6"
                v-model:value="state.codeParams.code"
                @change="handleCodeChange"
              />
              <a-button class="code-btn" type="primary" @click="handleGetCode">
                {{ state.countTime ? state.countNum : '获取验证码' }}
              </a-button>
            </div>
          </a-form-item>
          <a-form-item style="text-align: center">
            <a-button type="primary" class="login-btn">登录</a-button>
          </a-form-item>
        </a-form>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { onBeforeUnmount, reactive, ref } from 'vue';

const emit = defineEmits(['close']);

const state = reactive({
  params: {
    username: '123456',
    password: '',
    remember: false
  },
  codeParams: {
    cardNo: '',
    code: ''
  },
  countTime: null as any,
  countNum: 59
});

const showCodeLogin = ref(false);

const showModal = ref(true);
// 关闭模态框
const handleCloseModal = () => {
  emit('close');
};

// 验证码倒计时
const countDown = () => {
  if (state.countTime) {
    clearTimeout(state.countTime);
    state.countTime = null;
  }
  state.countTime = setTimeout(() => {
    state.countNum--;
    if (state.countNum >= 0) {
      countDown();
    } else {
      state.countNum = 59;
      state.countTime = null;
    }
  }, 1000);
};

// 获取验证码
const handleGetCode = () => {
  countDown();
};

// 证件号改变
const handleCardChange = () => {
  if (state.codeParams.cardNo) {
    state.codeParams.cardNo = state.codeParams.cardNo.replace(/\D/g, '');
  }
};

// 验证码改变
const handleCodeChange = () => {
  if (state.codeParams.code) {
    state.codeParams.code = state.codeParams.code.replace(/\D/g, '');
  }
};

onBeforeUnmount(() => {
  if (state.countTime) {
    clearTimeout(state.countTime);
    state.countTime = null;
  }
});
</script>

<style lang="scss" scoped>
.form-container {
  padding-top: 40px;
  .title {
    padding-bottom: 20px;
    text-align: center;
    font-size: 18px;
    font-weight: bold;
  }
  .login-btn {
    width: 100%;
  }
}
.code-btn {
  width: 150px;
  margin-left: 10px;
}
</style>
