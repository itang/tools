from .app import create_driver, MyDriver


def main() -> None:
    driver = create_driver(maximize_window=True)

    try:
        my_driver = MyDriver(driver)

        # 调用自动登录函数
        my_driver.auto_login(
            login_page="https://idemo.com/login.html",
        )

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
