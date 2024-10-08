pub const CREATE_USER_QUERY: &str = "
        BEGIN TRANSACTION;

        IF string::len($email) = 0 && string::len($phone) = 0 {
            RETURN 'email or phone required';
        } ELSE {
            LET $user = CREATE user CONTENT {
            	first_name: $first_name,
        	    last_name: $last_name,
                password: $password,
                password_salt: $password_salt,
                phone_code: $phone_code,
                created_on: time::now()
            };

            IF string::len($email) > 0 {
                LET $email_contact = CREATE contact CONTENT {
                    type: 'EMAIL',
                    value: $email
                };
                RELATE $user->owns_contact->$email_contact
                    SET is_email = true;
            };

            IF string::len($phone) > 0 {
                LET $phone_contact = CREATE contact CONTENT {
                    type: 'PHONE',
                    value: $phone
                };
                RELATE $user->owns_contact->$phone_contact
                    SET is_phone = true;
            };

            RETURN $user;
        };

        COMMIT TRANSACTION;
    ";