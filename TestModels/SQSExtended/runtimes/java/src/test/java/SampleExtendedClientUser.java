import org.testng.annotations.Test;
import polymorph.tutorial.sqsextended.SQSExtended;
import polymorph.tutorial.sqsextended.model.HandleMessagesInput;
import polymorph.tutorial.sqsextended.model.SQSExtendedClientConfig;
import software.amazon.awssdk.services.sqs.SqsClient;
import software.amazon.awssdk.services.sqs.model.ReceiveMessageRequest;

import java.util.UUID;

public class SampleExtendedClientUser {

    @Test
    public void testHandleMessages() {
        SqsClient sqsClient = SqsClient.builder().build();

        SQSExtendedClientConfig config = SQSExtendedClientConfig.builder()
                .sqsClient(sqsClient)
                .build();
        SQSExtended sqsExtendedClient = SQSExtended.builder()
                .SQSExtendedClientConfig(config)
                .build();

        String queueName = "TestHandleMessages-" + UUID.randomUUID();
        String queueUrl = sqsClient.createQueue(builder -> builder.queueName(queueName)).queueUrl();

        sqsClient.sendMessage(builder -> builder.queueUrl(queueUrl).messageBody("Hi there from Java!"));

        ReceiveMessageRequest receiveRequest = ReceiveMessageRequest.builder()
                .queueUrl(queueUrl)
                .build();

        sqsExtendedClient.HandleMessages(HandleMessagesInput.builder()
                .receiveRequest(receiveRequest)
                .handler(handlerInput ->
                    System.out.println("Got a message: " + handlerInput.message().body()))
                .build());
    }
}
