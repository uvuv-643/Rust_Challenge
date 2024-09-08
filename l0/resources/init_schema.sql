CREATE PROCEDURE init_schema()
LANGUAGE SQL
AS $$

    CREATE TABLE IF NOT EXISTS Orders (
        order_uid VARCHAR(255) PRIMARY KEY,
        track_number VARCHAR(255),
        entry VARCHAR(255),
        locale VARCHAR(255),
        internal_signature VARCHAR(255),
        customer_id VARCHAR(255),
        delivery_service VARCHAR(255),
        shardkey VARCHAR(255),
        sm_id INT,
        oof_shard VARCHAR(255),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );

    CREATE TABLE IF NOT EXISTS Delivery (
        id SERIAL PRIMARY KEY ,
        order_uid VARCHAR(255) REFERENCES Orders(order_uid),
        name VARCHAR(255),
        phone VARCHAR(255),
        zip VARCHAR(255),
        city VARCHAR(255),
        address VARCHAR(255),
        region VARCHAR(255),
        email VARCHAR(255),
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );

    CREATE TABLE IF NOT EXISTS Payment (
        order_uid VARCHAR(255) REFERENCES Orders(order_uid),
        "transaction" VARCHAR(255),
        request_id VARCHAR(255),
        currency VARCHAR(255),
        provider VARCHAR(255),
        amount INT,
        payment_dt timestamp,
        bank VARCHAR(255),
        delivery_cost INT,
        goods_total INT,
        custom_fee INT,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        PRIMARY KEY (order_uid)
    );

    CREATE TABLE IF NOT EXISTS Items (
        id SERIAL PRIMARY KEY,
        order_uid VARCHAR(255) REFERENCES Orders(order_uid),
        chrt_id INT,
        track_number VARCHAR(255),
        price INT,
        rid VARCHAR(255),
        name VARCHAR(255),
        sale INT,
        size VARCHAR(255),
        total_price INT,
        nm_id INT,
        brand VARCHAR(255),
        status INT,
        created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
    );

$$;
