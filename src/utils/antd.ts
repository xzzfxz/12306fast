import { App } from 'vue';
import {
  Modal,
  Form,
  Input,
  Button,
  Checkbox,
  Select,
  DatePicker,
  ConfigProvider,
  Table
} from 'ant-design-vue';

export const useAntD = {
  install(app: App) {
    app
      .use(Modal)
      .use(Button)
      .use(Form)
      .use(Checkbox)
      .use(Input)
      .use(Select)
      .use(DatePicker)
      .use(Table)
      .use(ConfigProvider);
  }
};

export default useAntD;
