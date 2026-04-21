This repository has beginner friendly code to learn rust axum web development.

The structure is as follows:
src folder has all the rust code and backend logic
templates folder is using askama to connect backend with html code

Functionality:
        1. User is taken to the main page where we take input
        2. User hits echo button
        3. The text entered is displayed. Also, the url is updated to /echo?text=xyx
        4. User can also fill the input and send post request
        5. The input text is still displayed but the url stays the same.

To run this app locally one must

        1. Clone this repository on their device
        2. In the terminal go to the repository folder
        3. Install dependencies mentioned in cargo.toml
            ``` cargo add axum ```
        4. Run ``` cargo build ```
        5. Run ``` cargo run ```. You can open the url which will run on the port mentioned in main.rs
        6. Instead of running, you can also test the code using ``` cargo test ``` this will run tests.rs file


New Functionality: 
        1. The echo post form can now take 3 inputs as text, number 1 and number 2. 
        2. The inputed values and their sum is displayed when post request is made and form is submitted
        3. TODO: Updating the UI and exploring more ways to update the form