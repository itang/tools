import time
from selenium import webdriver
from selenium.common.exceptions import NoSuchWindowException
from selenium.webdriver.remote.webdriver import WebDriver
from selenium.webdriver.common.by import By
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.remote.webelement import WebElement
from webdriver_manager.chrome import ChromeDriverManager


__all__ = ["MyDriver", "get_driver"]


def _try_get_element(driver: WebDriver, css_selector: str) -> WebElement | None:
    try:
        return driver.find_element(By.CSS_SELECTOR, css_selector)
    except:  # noqa: E722
        return None


def _get_button_or_wait(driver, wait_time: int = 2) -> WebElement:
    count = 0
    button = None

    while not button:
        # 等待元素加载
        driver.implicitly_wait(wait_time)
        print(f"INFO: [{count}] try get button...")
        button = _try_get_element(driver, css_selector=".login-btn")
        count += 1

    return button


def get_driver() -> WebDriver:
    # 设置WebDriver
    chrome_options = Options()
    # 尝试添加这些参数来隐藏被控制提示
    # TODO: 不起作用?
    chrome_options.add_experimental_option("detach", True)
    chrome_options.add_argument("--disable-infobars")

    return webdriver.Chrome(
        service=Service(ChromeDriverManager().install()), options=chrome_options
    )


class MyDriver:
    def __init__(self, driver: WebDriver):
        self.driver = driver

    def auto_login(self) -> None:
        button = _get_button_or_wait(self.driver)

        # 找到表单输入元素并填充数据
        username_input = self.driver.find_element(By.CSS_SELECTOR, ".login-input")
        username_input.send_keys("18018786450")

        password_input = self.driver.find_element(
            By.CSS_SELECTOR, 'input[type="password"]'
        )
        password_input.send_keys("Authine@123456")

        # 找到按钮并点击
        button.click()

        # # 等待页面加载
        # driver.implicitly_wait(10)  # 等待10秒

        print("INFO: Form submitted successfully.")

    def wait_browser_closed(self) -> None:
        while True:
            try:
                # 尝试获取窗口标题，如果窗口已关闭，将引发NoSuchWindowException
                self.driver.title
                time.sleep(1)  # 稍等一段时间再次检查
            except NoSuchWindowException:
                print("INFO: 窗口已关闭, 自动退出.")
                break
