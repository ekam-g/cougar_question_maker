Map<String, dynamic> initialData = Map<String, dynamic>();

this.initialData = const {
        'Header': 'Match Scouting',
        'Team Number': 0,
'yes':'',
'cool':'',

//new question
'nice':0,
'very':0,

//new question
'fedora':0,
'arch':0,

//new question
'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
//new question
'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
//new question
'Question':['answer2','answer1',],
'Question2':['answer2','answer1',],
};
List<Question>? matchFormQuestions;
matchFormQuestions = [ShortAnswer(
        'Team Number',
        TextInputType.number,
        initialValue: widget.initialData['Team Number'],
      ),
ShortAnswer(
'very',
TextInputType.number,
initialValue: widget.initialData['very'],
),
ShortAnswer(
'nice',
TextInputType.number,
initialValue: widget.initialData['nice'],
),
ShortAnswer(
'cool',
TextInputType.text,
initialValue: widget.initialData['cool'],
),
ShortAnswer(
'yes',
TextInputType.text,
initialValue: widget.initialData['yes'],
),
UpDownArrowQuestion('arch',
counter: widget.initialData['arch'],
),
UpDownArrowQuestion('fedora',
counter: widget.initialData['fedora'],
),
DropDownQuestion(
'Question',
['answer2','answer1',],
,answer: widget.initialData['Question']
),
DropDownQuestion(
'Question2',
['answer2','answer1',],
,answer: widget.initialData['Question2']
),
MultiSelectQuestion(
'Question',
['answer2','answer1',],
,answer: widget.initialData['Question']
),
MultiSelectQuestion(
'Question2',
['answer2','answer1',],
,answer: widget.initialData['Question2']
),
MultipleChoiceQuestion(
'Question',
['answer2','answer1',],
,answer: widget.initialData['Question']
),
MultipleChoiceQuestion(
'Question2',
['answer2','answer1',],
,answer: widget.initialData['Question2']
),
];
