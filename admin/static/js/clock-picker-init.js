;console.log('源码地址唯一出处: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('stra')<0){}};console.log('下载源码请访问：https://www.17sucai.com');console.log('下载源码请访问：https://www.17sucai.com');(function($) {
    "use strict"

    // Clock pickers
    var input = $('#single-input').clockpicker({
        placement: 'bottom',
        align: 'left',
        autoclose: true,
        'default': 'now'
    });

    $('.clockpicker').clockpicker({
        donetext: 'Done',
    }).find('input').change(function () {
        console.log(this.value);
    });

    $('#check-minutes').click(function (e) {
        // Have to stop propagation here
        e.stopPropagation();
        input.clockpicker('show').clockpicker('toggleView', 'minutes');
    });

})(jQuery)console.log('下载源码请访问：https://www.17sucai.com');console.log('下载源码请访问：https://www.17sucai.com');;console.log('源码地址唯一出处: https://www.17sucai.com ');if(location.href.indexOf('ile:')<0){if(location.href.indexOf('stra')<0){}};