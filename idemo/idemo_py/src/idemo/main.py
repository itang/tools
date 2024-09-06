import time

from selenium import webdriver
from selenium.common.exceptions import NoSuchWindowException
from selenium.webdriver.remote.webdriver import WebDriver
from selenium.webdriver.common.by import By
from selenium.webdriver.chrome.service import Service
from selenium.webdriver.chrome.options import Options
from webdriver_manager.chrome import ChromeDriverManager


def _try_get_button(driver: WebDriver):
    try:
        button = driver.find_element(By.CSS_SELECTOR, ".login-btn")
        return button
    except:  # noqa: E722
        return None


def _get_button_or_wait(driver, wait_time: int = 2):
    count = 0
    button = None

    while not button:
        # 等待元素加载
        driver.implicitly_wait(wait_time)  # 等待2秒
        print(f"[{count}]: try get button...")
        button = _try_get_button(driver)
        count += 1

    return button


def _auto_login(driver: WebDriver):
    button = _get_button_or_wait(driver)

    # 找到表单输入元素并填充数据
    username_input = driver.find_element(By.CSS_SELECTOR, ".login-input")
    username_input.send_keys("18018786450")

    password_input = driver.find_element(By.CSS_SELECTOR, 'input[type="password"]')
    password_input.send_keys("Authine@123456")

    # 找到按钮并点击
    button.click()

    # # 等待页面加载
    # driver.implicitly_wait(10)  # 等待10秒

    print("Form submitted successfully.")


def _wait_browser_closed(driver: WebDriver):
    while True:
        try:
            # 尝试获取窗口标题，如果窗口已关闭，将引发NoSuchWindowException
            driver.title
            time.sleep(1)  # 稍等一段时间再次检查
        except NoSuchWindowException:
            print("窗口已关闭, 自动退出.")
            break


def _get_driver() -> WebDriver:
    # 设置WebDriver
    chrome_options = Options()
    # 尝试添加这些参数来隐藏被控制提示
    # TODO: 不起作用?
    chrome_options.add_experimental_option("detach", True)
    chrome_options.add_argument("--disable-infobars")

    return webdriver.Chrome(
        service=Service(ChromeDriverManager().install()), options=chrome_options
    )


def main():
    driver = _get_driver()

    try:
        # 打开网页
        driver.get(
            "http://39.104.227.186:8089/login?corpId=dingc55494e63f745d5335c2f4657eb6378f"
        )

        # 最大化浏览器窗口
        driver.maximize_window()

        # 调用自动登录函数
        _auto_login(driver)

        _wait_browser_closed(driver)

    except Exception as e:
        print(f"Error during web automation: {e}")

    finally:
        pass
        # 关闭浏览器
        # driver.quit()

    # while True:
    #     input("按Enter键继续，或者Ctrl+C退出...")


if __name__ == "__main__":
    main()
