






/// Start a new interface
/// 
pub async fn start_interface<A: Into<String>, B: Into<String>>
    (
        builder: InterfaceBuilder, 
        dev_name: A, 
        bench_name: B, 
        connection_link_manager: AmLinkManager,
        platform_services: crate::platform::services::AmServices
    )
        -> AmRunner
{
    let _dev_name = dev_name.into();
    let _bench_name = bench_name.into();

    // Topic name
    let topic = format!("pza/{}/{}/{}", _bench_name, _dev_name, builder.name);

    // Get attributes names
    let att_names = builder.subscriber.attributes_names().await;

    // Build subscriptions requests
    let mut requests = vec![
        subscription::Request::new( subscription::ID_PZA, "pza" ),
        subscription::Request::new( subscription::ID_PZA_CMDS_SET, &format!("{}/cmds/set", topic) )
    ];
    for att_name in att_names {
        let request = subscription::Request::new( att_name.0, &format!("{}/cmds/{}", topic, att_name.1) );
        requests.push(request);
    }

    // Create the link with the connection
    let link = connection_link_manager.lock().await.request_link(requests).await.unwrap();
    
    
}
