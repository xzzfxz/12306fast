<template>
  <a-modal
    v-model:open="showModal"
    :footer="null"
    :maskClosable="false"
    @cancel="handleCloseModal">
    <div class="form-container">
      <div class="title">{{ showCodeLogin ? '短信验证' : '密码登录' }}</div>
      <div class="login-form" v-show="!showCodeLogin">
        <a-form
          :model="state.params"
          name="loginForm"
          size="large"
          :label-col="{ span: 4 }"
          :wrapper-col="{ span: 16 }"
          autocomplete="off">
          <a-form-item
            label="用户名"
            name="username"
            :rules="[{ required: true, message: '请输入用户名!' }]">
            <a-input v-model:value="state.params.username" />
          </a-form-item>
          <a-form-item
            label="密码"
            name="password"
            :rules="[{ required: true, message: '请输入密码!' }]">
            <a-input-password
              v-model:value="state.params.password"></a-input-password>
          </a-form-item>

          <a-form-item name="remember" :wrapper-col="{ offset: 4, span: 16 }">
            <a-checkbox v-model:checked="state.params.remember">
              记住密码
            </a-checkbox>
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
          autocomplete="off">
          <a-form-item
            label=""
            name="cardNo"
            :rules="[
              { required: true, message: '请输入登录账号绑定的证件号后4位!' },
            ]">
            <a-input
              placeholder="请输入登录账号绑定的证件号后4位"
              v-model:value="state.codeParams.cardNo" />
          </a-form-item>
          <a-form-item
            label=""
            name="code"
            :rules="[{ required: true, message: '请输入验证码!' }]">
            <div class="flex code-container">
              <a-input
                placeholder="输入验证码"
                v-model:value="state.codeParams.code"></a-input>
              <a-button class="code-btn" type="primary">获取验证码</a-button>
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
import { reactive, ref } from 'vue';

const emit = defineEmits(['close']);

const state = reactive({
  params: {
    username: '123456',
    password: '',
    remember: false,
  },
  codeParams: {
    cardNo: '',
    code: '',
  },
  passwordType: 'password',
});

const showCodeLogin = ref(true);

const showModal = ref(true);
// 关闭模态框
const handleCloseModal = () => {
  emit('close');
};
</script>

<style lang="scss" scoped>
.form-container {
  border: 1px solid red;
  padding-top: 40px;
  :deep(.ant-input) {
    box-shadow: none;
  }
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
  margin-left: 10px;
}
</style>
