from .app import get_driver, MyDriver


def main() -> None:
    driver = get_driver()

    try:
        page = "http://39.104.227.186:8089/login?corpId=dingc55494e63f745d5335c2f4657eb6378f"

        # 打开网页
        driver.get(page)

        # 最大化浏览器窗口
        driver.maximize_window()

        my_driver = MyDriver(driver)

        # 调用自动登录函数
        my_driver.auto_login()

        # 等待浏览器关闭
        my_driver.wait_browser_closed()

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
