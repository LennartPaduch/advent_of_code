var ajaxCall = (key, url, messages) => {
    return new Promise((resolve, reject) => {
        $.ajax({
            url: url,
            type: "POST",
            dataType: "json",
            data: JSON.stringify({
                model: "gpt-3.5-turbo",
                messages: messages,
                max_tokens: 256,
                temperature: 0.5,
                top_p: 1,
                frequency_penalty: 0,
                presence_penalty: 0
            }),
            headers: {
                "Content-Type": "application/json",
                Authorization: `Bearer ${key}`
            },
            crossDomain: true,
            success: function (response, status, xhr) {
                resolve({ response, status, xhr });
            },
            error: function (xhr, status, error) {
                const err = new Error('xhr error');
                err.status = xhr.status;
                reject(err);
            },
        });
    });
};

const url = "https://api.openai.com/v1/chat/completions";

(function () {
    const template = document.createElement("template");
    template.innerHTML = `
        <style>
        </style>
        <div id="root" style="width: 100%; height: 100%;">
        </div>
      `;
    class MainWebComponent extends HTMLElement {
        async post(apiKey, prompt) {
            const messages = [
                {
                    role: "user",
                    content: prompt
                }
            ];
            const { response } = await ajaxCall(
                apiKey,
                url,
                messages
            );
            return response.choices[0].message.content;
        }
    }
    customElements.define("custom-widget", MainWebComponent);
})();

console.log("test")