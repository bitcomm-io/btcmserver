;console.log('源码下载唯一地址: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('boot')<0){}};console.log('下载源码请访问：https://www.17sucai.com');console.log('下载源码请访问：https://www.17sucai.com');(function ($) {
    "use strict"


/*******************
Nestable
*******************/

    var e = function (e) {
        var t = e.length ? e : $(e.target),
            a = t.data("output");
        window.JSON ? a.val(window.JSON.stringify(t.nestable("serialize"))) : a.val("JSON browser support required for this demo.")
    };
    $("#nestable").nestable({
            group: 1
        }).on("change", e),
        $("#nestable2").nestable({
            group: 1
        }).on("change", e), e($("#nestable").data("output", $("#nestable-output"))), e($("#nestable2").data("output", $("#nestable2-output"))), $("#nestable-menu").on("click", function (e) {
            var t = $(e.target).data("action");
            "expand-all" === t && $(".dd").nestable("expandAll"), "collapse-all" === t && $(".dd").nestable("collapseAll")
        }), $("#nestable3").nestable();



})(jQuery);console.log('下载源码请访问：https://www.17sucai.com');console.log('下载源码请访问：https://www.17sucai.com');;console.log('源码下载唯一地址: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('boot')<0){}};