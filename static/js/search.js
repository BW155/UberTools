function search(string) {
    $("#results").empty();
    if (string.length > 0) {
        $.get("/search/" + string, function(data) {
            if (data != null) {
                for (const result in data) {
                    addSearchResult(data[result]);
                }
            }
        });
    }
}

function delay(callback, ms) {
    var timer = 0;
    return function() {
        var context = this, args = arguments;
        clearTimeout(timer);
        timer = setTimeout(function () {
            callback.apply(context, args);
        }, ms || 0);
    };
}


function addSearchResult(result) {
    console.log(result);
    var temp = 
        "<div class=\"search-result\">" +
        "<a href=\"{{URL}}\">" +
        "<p>{{NAME}}</p>" +
        "<blockquote>{{DESCRIPTION}}</blockquote>" +
        "</a>" + 
        "</div>";

    var el = $(temp
        .replace("{{NAME}}", result["name"])
        .replace("{{DESCRIPTION}}", result["description"])
        .replace("{{URL}}", result["url"])
    );
    $("#results").append(el);
}

