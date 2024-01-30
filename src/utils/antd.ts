import { App } from 'vue';
import { Modal, Form, Input, Button, Checkbox } from 'ant-design-vue';

export const useAntD = {
  install(app: App) {
    app.use(Modal).use(Button).use(Form).use(Checkbox).use(Input);
  },
};

export default useAntD;
